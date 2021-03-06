/*
 *  This software is a computer program whose purpose is to compute validitiy of
 *  identification data.
 *
 *  Copyright (C) 2017 Th!nk Inc.
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, only version 2.0.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License along
 *  with this program; if not, write to the Free Software Foundation, Inc.,
 *  51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 */

/* Original LibreAuth License */

/*
 * Copyright Rodolphe Breard (2015)
 * Author: Rodolphe Breard (2015)
 *
 * This software is a computer program whose purpose is to [describe
 * functionalities and technical features of your software].
 *
 * This software is governed by the CeCILL  license under French law and
 * abiding by the rules of distribution of free software.  You can  use,
 * modify and/ or redistribute the software under the terms of the CeCILL
 * license as circulated by CEA, CNRS and INRIA at the following URL
 * "http://www.cecill.info".
 *
 * As a counterpart to the access to the source code and  rights to copy,
 * modify and redistribute granted by the license, users are provided only
 * with a limited warranty  and the software's author,  the holder of the
 * economic rights,  and the successive licensors  have only  limited
 * liability.
 *
 * In this respect, the user's attention is drawn to the risks associated
 * with loading,  using,  modifying and/or developing or reproducing the
 * software by the user in light of its specific status of free software,
 * that may mean  that it is complicated to manipulate,  and  that  also
 * therefore means  that it is reserved for developers  and  experienced
 * professionals having in-depth computer knowledge. Users are therefore
 * encouraged to load and test the software's suitability as regards their
 * requirements in conditions enabling the security of their systems and/or
 * data to be ensured and,  more generally, to use and operate it in the
 * same conditions as regards security.
 *
 * The fact that you are presently reading this means that you have had
 * knowledge of the CeCILL license and that you accept its terms.
 */


//!
//! [![Build Status](https://api.travis-ci.org/breard-r/boringauth.svg?branch=master)](https://travis-ci.org/breard-r/boringauth)
//! [![LibreAuth on crates.io](https://img.shields.io/crates/v/boringauth.svg)](https://crates.io/crates/boringauth)
//!
//! LibreAuth is a collection of tools for user authentication.
//!
//!
//! ## Features
//! - HOTP - HMAC-based One-time Password Algorithm ([RFC 4226](https://tools.ietf.org/html/rfc4226))
//!   - the key can be passed as bytes, an ASCII string, an hexadicimal string or a base32 string
//!   - customizable counter
//!   - customizable hash function (sha1, sha256, sha512)
//!   - customizable output length
//!   - customizable output alphabet
//! - TOTP - Time-based One-time Password Algorithm ([RFC 6238](https://tools.ietf.org/html/rfc6238))
//!   - the key can be passed as bytes, an ASCII string, an hexadicimal string or a base32 string
//!   - customizable timestamp
//!   - customizable period
//!   - customizable initial time (T0)
//!   - customizable hash function (sha1, sha256, sha512)
//!   - customizable output length
//!   - customizable output alphabet
//!

extern crate base32;
extern crate ring;
extern crate rand;
extern crate time;
extern crate hex;

pub mod oath;
pub mod pass;
mod parser;

#[cfg(feature = "cbindings")]
extern crate libc;
