use super::{read_from_file, write_to_file};
use crate::npe;
use crate::operators::crypto;
use crate::pro_api;
use crate::pro_api::error::ProAPIError;
use crate::pro_api::Torus;
use crate::Types;
use backtrace::Backtrace;
use colored::Colorize;
use itertools::izip;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Structure describing one particular Encoding
/// # Attributes
/// * `o` - the offset of the encoding
/// * `delta` - the delta of the encoding
/// * `nb_bit_precision` - the minimum number of bits to represent a plaintext
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Encoder {
    pub o: f64,     // with margin between 1 and 0
    pub delta: f64, // with margin between 1 and 0
    pub nb_bit_precision: usize,
    pub nb_bit_padding: usize,
}

impl Encoder {
    /// Instantiate a new Encoder with the provided interval as [min,max[
    /// # Arguments
    /// * `min`- the minimum real value of the interval
    /// * `max`- the maximum real value of the interval
    /// * `nb_bit_precision` - number of bits to represent a plaintext
    /// * `nb_bit_padding` - number of bits for left padding with zeros
    /// # Output
    /// * a new instantiation of an Encoder
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let min: f64 = 0.2;
    /// let max: f64 = 0.8;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    ///
    /// // instantiation
    /// let encoder = Encoder::new(min, max, nb_bit_precision, nb_bit_padding).unwrap();
    /// ```
    pub fn new(
        min: f64,
        max: f64,
        nb_bit_precision: usize,
        nb_bit_padding: usize,
    ) -> Result<Encoder, ProAPIError> {
        if min >= max {
            return Err(MinMaxError!(min, max));
        }
        if nb_bit_precision == 0 {
            return Err(PrecisionError!());
        }

        let margin: f64 = (max - min) / (f64::powi(2., nb_bit_precision as i32) - 1.);

        Ok(Encoder {
            o: min,
            delta: max - min + margin,
            nb_bit_precision: nb_bit_precision,
            nb_bit_padding: nb_bit_padding,
        })
    }

    /// After an homomorphic operation, update an encoder using the variance
    /// # Arguments
    /// * `variance` - variance
    /// # Output
    /// * return the number of bits of precision affected by the noise
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let min: f64 = 0.2;
    /// let max: f64 = 0.8;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    ///
    /// // instantiation
    /// let mut encoder = Encoder::new(min, max, nb_bit_precision, nb_bit_padding).unwrap();
    /// let variance: f64 = f64::powi(2., -30);
    /// let nb_bit_overlap: usize = encoder.update_precision_from_variance(variance).unwrap();
    /// ```
    pub fn update_precision_from_variance(&mut self, variance: f64) -> Result<usize, ProAPIError> {
        // check output noise
        let nb_noise_bit: usize =
            npe::nb_bit_from_variance_99(variance, <Torus as Types>::TORUS_BIT);

        // check if there actually some noise in the ciphertext
        if nb_noise_bit == 0 {
            return Err(NoNoiseInCiphertext!(variance));
        } else if nb_noise_bit + self.nb_bit_precision + self.nb_bit_padding
            > <Torus as Types>::TORUS_BIT
        {
            // compute the number of bits which can be overwritten by the noise
            let nb_bit_overlap = nb_noise_bit + self.nb_bit_precision + self.nb_bit_padding
                - <Torus as Types>::TORUS_BIT;

            // if the overlap is at least as big as the precision,
            // there is no more message
            self.nb_bit_precision =
                i32::max(self.nb_bit_precision as i32 - nb_bit_overlap as i32, 0i32) as usize;

            Ok(nb_bit_overlap)
        } else {
            Ok(0)
        }
    }

    /// Instantiate a new Encoder with the provided interval as [center-radius,center+radius[
    /// # Arguments
    /// * `center` - the center value of the interval
    /// * `radius` - the distance between the center and the endpoints of the interval
    /// * `nb_bit_precision` - number of bits to represent a plaintext
    /// * `nb_bit_padding` - number of bits for left padding with zeros
    /// # Output
    /// * a new instantiation of an Encoder
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let center: f64 = 0.;
    /// let radius: f64 = 5.4;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    ///
    /// // instantiation
    /// let encoder = Encoder::new_centered(center, radius, nb_bit_precision, nb_bit_padding).unwrap();
    /// ```
    pub fn new_centered(
        center: f64,
        radius: f64,
        nb_bit_precision: usize,
        nb_bit_padding: usize,
    ) -> Result<Encoder, ProAPIError> {
        if radius <= 0. {
            return Err(RadiusError!(radius));
        }
        if nb_bit_precision == 0 {
            return Err(PrecisionError!());
        }
        Ok(pro_api::Encoder::new(
            center - radius,
            center + radius,
            nb_bit_precision,
            nb_bit_padding,
        )?)
    }

    /// Encode one single message according to this Encoder parameters
    /// # Arguments
    /// * `message` - a message as a f64
    /// # Output
    /// * a new instantiation of an Plaintext containing only one encoded value (the one we just computed with this function)
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let min: f64 = 0.2;
    /// let max: f64 = 0.8;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    /// let message = 0.6;
    ///
    /// // creation of the encoder
    /// let encoder = Encoder::new(min, max, nb_bit_precision, nb_bit_padding).unwrap();
    ///
    /// // encoding
    /// let m = encoder.encode_single(message).unwrap();
    /// ```
    pub fn encode_single(&self, message: f64) -> Result<pro_api::Plaintext, ProAPIError> {
        if message < self.o || message > self.o + self.delta {
            return Err(MessageOutsideIntervalError!(message, self.o, self.delta));
        }
        Ok(pro_api::Plaintext {
            encoders: vec![self.clone(); 1],
            plaintexts: vec![self.encode_operators(message)?; 1],
            nb_plaintexts: 1,
        })
    }

    /// Decode one single plaintext according to this Encoder parameters
    /// # Arguments
    /// * `ec` - an plaintext
    /// # Output
    /// * the decoded value as a f64
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let min: f64 = 0.2;
    /// let max: f64 = 0.8;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    /// let message = 0.6;
    ///
    /// // creation of the encoder
    /// let encoder = Encoder::new(min, max, nb_bit_precision, nb_bit_padding).unwrap();
    ///
    /// // encoding
    /// let m = encoder.encode_single(message).unwrap();
    ///
    /// // decoding
    /// let new_message = encoder.decode_single(m.plaintexts[0]).unwrap();
    /// ```
    pub fn decode_single(&self, ec: Torus) -> Result<f64, ProAPIError> {
        return self.decode_operators(ec);
    }

    /// Instantiate a new empty Encoder (set to zero)
    /// # Output
    /// * a new instantiation of an empty Encoder (set to zero)
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    /// let encoder = Encoder::zero();
    /// ```
    pub fn zero() -> Encoder {
        Encoder {
            o: 0.,
            delta: 0.,
            nb_bit_precision: 0,
            nb_bit_padding: 0,
        }
    }

    /// Encode several message according to this (one) Encoder parameters
    /// The output Plaintext will have plaintexts all computed with the same Encoder parameters
    /// # Arguments
    /// * `messages`- a list of messages as a f64
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    /// // parameters
    /// let (min, max): (f64, f64) = (0.2, 0.4);
    /// let (precision, padding): (usize, usize) = (8, 4);
    /// let messages: Vec<f64> = vec![0.3, 0.34];
    /// let encoder = Encoder::new(min, max, precision, padding).unwrap();
    /// let plaintexts = encoder.encode(&messages).unwrap();
    /// ```
    pub fn encode(&self, messages: &[f64]) -> Result<pro_api::Plaintext, ProAPIError> {
        let mut result = pro_api::Plaintext {
            encoders: vec![self.clone(); messages.len()],
            plaintexts: vec![0 as Torus; messages.len()],
            nb_plaintexts: messages.len(),
        };
        debug_assert!(
            result.plaintexts.len() == result.encoders.len(),
            "result.plaintexts.len() != result.encoders.len()"
        );
        debug_assert!(
            result.plaintexts.len() == messages.len(),
            "result.plaintexts.len() != messages.len()"
        );
        for (pt, encoder, m) in izip!(
            result.plaintexts.iter_mut(),
            result.encoders.iter_mut(),
            messages.iter()
        ) {
            *pt = self.encode_operators(*m)?;
            encoder.copy(self);
        }
        Ok(result)
    }

    /// Computes the smallest real number that this encoding can handle
    pub fn get_granularity(&self) -> f64 {
        return self.delta / f64::powi(2., self.nb_bit_precision as i32);
    }

    pub fn get_min(&self) -> f64 {
        return self.o;
    }

    pub fn get_max(&self) -> f64 {
        return self.o + self.delta - self.get_granularity();
    }

    pub fn get_size(&self) -> f64 {
        return self.delta - self.get_granularity();
    }

    /// Copy the content of the input encoder inside the self encoder
    /// # Argument
    /// * `encoder`- the encoder to be copied
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    /// // parameters
    /// let (min, max): (f64, f64) = (0.2, 0.4);
    /// let (precision, padding): (usize, usize) = (8, 4);
    ///
    /// let encoder_1 = Encoder::new(min, max, precision, padding).unwrap();
    /// let mut encoder_2 = Encoder::zero();
    /// encoder_2.copy(&encoder_1);
    /// ```
    pub fn copy(&mut self, encoder: &pro_api::Encoder) {
        self.o = encoder.o;
        self.delta = encoder.delta;
        self.nb_bit_precision = encoder.nb_bit_precision;
        self.nb_bit_padding = encoder.nb_bit_padding;
    }

    /// Crete a new encoder as if one computes a square function divided by 4
    /// # Argument
    /// * `nb_bit_padding`- number of bits for left padding with zeros
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let min: f64 = 0.2;
    /// let max: f64 = 0.8;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    ///
    /// // instantiation
    /// let encoder_in = Encoder::new(min, max, nb_bit_precision, nb_bit_padding).unwrap();
    /// let encoder_out = encoder_in
    ///     .new_square_divided_by_four(nb_bit_padding)
    ///     .unwrap();
    /// ```
    pub fn new_square_divided_by_four(
        &self,
        nb_bit_padding: usize,
    ) -> Result<pro_api::Encoder, ProAPIError> {
        // valid encoder
        if !self.is_valid() {
            return Err(InvalidEncoderError!(self.nb_bit_precision, self.delta));
        }
        if self.nb_bit_padding < 1 {
            return Err(NotEnoughPaddingError!(self.nb_bit_padding, 1));
        }

        if self.o < 0. && self.o + self.delta < 0. {
            // only negative values in the interval
            let new_max = (self.o * self.o) / 4.;
            let old_max = self.o + self.delta - self.get_granularity();
            let new_min = (old_max * old_max) / 4.;
            Ok(pro_api::Encoder::new(
                new_min,
                new_max,
                self.nb_bit_precision,
                nb_bit_padding,
            )?)
        } else if self.o > 0. {
            // only positive values in the interval
            let new_min = (self.o * self.o) / 4.;
            let old_max = self.o + self.delta - self.get_granularity();
            let new_max = (old_max * old_max) / 4.;
            Ok(pro_api::Encoder::new(
                new_min,
                new_max,
                self.nb_bit_precision,
                nb_bit_padding,
            )?)
        } else {
            // 0 is in the interval
            let new_min: f64 = 0.;
            let old_max = self.o + self.delta - self.get_granularity();
            let max = old_max.max(-self.o);
            let new_max = max * max / 4.;
            Ok(pro_api::Encoder::new(
                new_min,
                new_max,
                self.nb_bit_precision,
                nb_bit_padding,
            )?)
        }
    }

    /// Wrap the operators encode function with the padding
    /// # Argument
    /// * `m` - the message to encode
    /// # Example
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let min: f64 = 0.2;
    /// let max: f64 = 0.8;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    ///
    /// // message
    /// let m = 0.3;
    /// // instantiation
    /// let encoder = Encoder::new(min, max, nb_bit_precision, nb_bit_padding).unwrap();
    ///
    /// let plaintext = encoder.encode_operators(m).unwrap();
    /// ```
    pub fn encode_operators(&self, m: f64) -> Result<Torus, ProAPIError> {
        if m < self.o || m >= self.o + self.delta {
            return Err(MessageOutsideIntervalError!(m, self.o, self.delta));
        }
        return self.encode_outside_interval_operators(m);
    }

    /// Wrap the operators encode function with the padding and allows to encode a message that is outside of the interval of the encoder
    /// It is used for correction after homomorphic computation
    /// # Argument
    /// * `m` - the message to encode
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let min: f64 = 0.2;
    /// let max: f64 = 0.8;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    ///
    /// // message
    /// let m = 1.2;
    /// // instantiation
    /// let encoder = Encoder::new(min, max, nb_bit_precision, nb_bit_padding).unwrap();
    ///
    /// let plaintext = encoder.encode_outside_interval_operators(m).unwrap();
    /// ```
    pub fn encode_outside_interval_operators(&self, m: f64) -> Result<Torus, ProAPIError> {
        if !self.is_valid() {
            return Err(InvalidEncoderError!(self.nb_bit_precision, self.delta));
        }
        let mut res = crypto::Encoding::encode(m, self.o, self.delta);
        if self.nb_bit_padding > 0 {
            res = res >> self.nb_bit_padding;
        }
        return Ok(res);
    }

    /// Wrap the operators decode function with the padding and the rounding
    /// # Argument
    /// * `pt` - the noisy plaintext
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let min: f64 = 0.2;
    /// let max: f64 = 0.8;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    ///
    /// // message
    /// let m = 0.3;
    /// // instantiation
    /// let encoder = Encoder::new(min, max, nb_bit_precision, nb_bit_padding).unwrap();
    ///
    /// let plaintext = encoder.encode_operators(m).unwrap();
    /// let new_message = encoder.decode_operators(plaintext).unwrap();
    /// ```
    pub fn decode_operators(&self, pt: Torus) -> Result<f64, ProAPIError> {
        if !self.is_valid() {
            return Err(InvalidEncoderError!(self.nb_bit_precision, self.delta));
        }
        let mut tmp =
            Types::round_to_closest_multiple(pt, self.nb_bit_precision + self.nb_bit_padding, 1);

        if self.nb_bit_padding > 0 {
            tmp = tmp << self.nb_bit_padding;
        }
        // let padding: f64 = self.delta / f64::powi(2., self.nb_bit_precision as i32 + 1);

        return Ok(crypto::Encoding::decode(tmp, self.o, self.delta));
    }

    /// Check if the Encoder looks valid or not
    /// # Output
    /// return a boolean, true means that it is valid
    pub fn is_valid(&self) -> bool {
        if self.nb_bit_precision == 0 {
            return false;
        } else if self.delta <= 0. {
            return false;
        }
        return true;
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        write_to_file(path, self)
    }

    pub fn load(path: &str) -> Result<Encoder, Box<dyn Error>> {
        read_from_file(path)
    }

    /// Modify the encoding to be use after an homomorphic opposite
    /// ```rust
    /// use concrete_lib::pro_api::Encoder;
    ///
    /// // parameters
    /// let min: f64 = 0.2;
    /// let max: f64 = 0.8;
    /// let nb_bit_precision = 8;
    /// let nb_bit_padding = 4;
    ///
    /// // message
    /// let m = 0.3;
    /// // instantiation
    /// let mut encoder = Encoder::new(min, max, nb_bit_precision, nb_bit_padding).unwrap();
    ///
    /// encoder.opposite_inplace();
    /// ```
    pub fn opposite_inplace(&mut self) -> Result<(), ProAPIError> {
        let old_max = self.o + self.delta - self.get_granularity();
        let new_o = -old_max;
        self.o = new_o;
        Ok(())
    }
}

/// Print needed pieces of information about an Encoder
impl fmt::Display for Encoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            " Encoder {{
            -> [{},{}[
            -> center = {}
            -> radius = {}
            -> nb bit precision = {}
            -> granularity = {}
            -> nb bit padding = {}
        }}
            ",
            self.o,
            self.o + self.delta,
            self.o + self.delta / 2.,
            self.delta / 2.,
            self.nb_bit_precision,
            self.get_granularity(),
            self.nb_bit_padding
        )
    }
}

#[cfg(test)]
mod tests;
