// Copyright 2018 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Data for testing parsing/serialization of ICMP.
//!
//! This data was obtained by capturing live network traffic.

pub(crate) mod ndp_neighbor {
    pub(crate) const SOLICITATION_IP_PACKET_BYTES: &[u8] = &[
        0x68, 0x00, 0x00, 0x00, 0x00, 0x20, 0x3a, 0xff, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x56, 0xe0, 0x32, 0x09, 0xc4, 0x74, 0x77, 0xf0, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x4f, 0xe7, 0x1a, 0x69, 0x86, 0x4b, 0x85, 0xc2, 0x87, 0x00, 0xca, 0xd0, 0x00,
        0x00, 0x00, 0x00, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4f, 0xe7, 0x1a, 0x69,
        0x86, 0x4b, 0x85, 0xc2, 0x01, 0x01, 0x54, 0xe0, 0x32, 0x74, 0x77, 0xf0,
    ];

    pub(crate) const SOURCE_LINK_LAYER_ADDRESS: &[u8] = &[0x54, 0xe0, 0x32, 0x74, 0x77, 0xf0];

    pub(crate) const TARGET_ADDRESS: &[u8] = &[
        0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4f, 0xe7, 0x1a, 0x69, 0x86, 0x4b, 0x85,
        0xc2,
    ];

    pub(crate) const ADVERTISEMENT_IP_PACKET_BYTES: &[u8] = &[
        0x60, 0x00, 0x00, 0x00, 0x00, 0x18, 0x3a, 0xff, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x4f, 0xe7, 0x1a, 0x69, 0x86, 0x4b, 0x85, 0xc2, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x56, 0xe0, 0x32, 0x09, 0xc4, 0x74, 0x77, 0xf0, 0x88, 0x00, 0x8a, 0x1e, 0x40,
        0x00, 0x00, 0x00, 0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4f, 0xe7, 0x1a, 0x69,
        0x86, 0x4b, 0x85, 0xc2,
    ];
}

pub(crate) mod ndp_router {
    use core::num::NonZeroU8;

    use const_unwrap::const_unwrap_option;
    use net_types::ip::{Ipv6Addr, Subnet};

    use crate::icmp::ndp::RoutePreference;
    use crate::utils::NonZeroDuration;

    #[rustfmt::skip]
    pub(crate) const ADVERTISEMENT_IP_PACKET_BYTES: &[u8] = &[
        // IPv6 Header - Start
        0x68, 0x00, 0x00, 0x00,
        0x00, 0x68, 0x3a, 0xff,
        0xfe, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x02, 0x00, 0xd0, 0x96,
        0xfe, 0x00, 0x02, 0x65,
        0xff, 0x02, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        // IPv6 Header - End
        // Router Advertisement Message - Start
        0x86, 0x00, 0xf9, 0x07,
        0x40, 0x00, 0x0e, 0x10,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        // Source Link-layer Address Option - Start
        0x01, 0x01, 0x00, 0x00,
        0x5e, 0x00, 0x02, 0x65,
        // Source Link-layer Address Option - End
        // Prefix information Option - Start
        0x03, 0x04, 0x40, 0xc0,
        0x00, 0x27, 0x8d, 0x00,
        0x00, 0x09, 0x3a, 0x80,
        0x00, 0x00, 0x00, 0x00,
        0x26, 0x20, 0x00, 0x00,
        0x10, 0x00, 0x50, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        // Prefix information Option- End
        // Route information Option High Pref - Start
        0x18, 0x03, 0x70, 0x08,
        0x00, 0x27, 0x8d, 0x00,
        0x26, 0x20, 0x10, 0x12,
        0x10, 0x00, 0x50, 0x00,
        0xa0, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        // Route information Option High Pref - End
        // Route information Option Medium Pref - Start
        0x18, 0x02, 0x38, 0x00,
        0x00, 0x09, 0x3a, 0x80,
        0x26, 0x20, 0x10, 0x12,
        0x10, 0x00, 0x00, 0x00,
        // Route information Option Medium Pref - End
        // Route information Option Low Pref - Start
        0x18, 0x01, 0x00, 0x18,
        0x00, 0x09, 0x3a, 0x80,
        // Route information Option Low Pref - End
        // Router Advertisement Message - End
    ];

    /// Options in the Advertisement packet.
    pub(crate) const HOP_LIMIT: Option<NonZeroU8> = Some(const_unwrap_option(NonZeroU8::new(64)));
    pub(crate) const LIFETIME: Option<NonZeroDuration> =
        Some(const_unwrap_option(NonZeroDuration::from_secs(3600)));
    pub(crate) const REACHABLE_TIME: Option<NonZeroDuration> = None;
    pub(crate) const RETRANS_TIMER: Option<NonZeroDuration> = None;

    /// Data from the SourceLinkLayerAddress option.
    pub(crate) const SOURCE_LINK_LAYER_ADDRESS: &[u8] = &[0x00, 0x00, 0x5e, 0x00, 0x02, 0x65];

    /// Data from the Prefix Info option.
    pub(crate) const PREFIX_INFO_ON_LINK_FLAG: bool = true;
    pub(crate) const PREFIX_INFO_AUTONOMOUS_ADDRESS_CONFIGURATION_FLAG: bool = true;
    pub(crate) const PREFIX_INFO_VALID_LIFETIME_SECONDS: u32 = 2_592_000;
    pub(crate) const PREFIX_INFO_PREFERRED_LIFETIME_SECONDS: u32 = 604_800;
    // We know this is safe as the none of the host-bits are set and the prefix
    // length is valid for IPv6.
    pub(crate) const PREFIX_INFO_PREFIX: Subnet<Ipv6Addr> = unsafe {
        Subnet::new_unchecked(
            Ipv6Addr::new([0x2620, 0x00, 0x1000, 0x5000, 0x00, 0x00, 0x00, 0x00]),
            64,
        )
    };

    /// Data from the Route Information options
    pub(crate) const ROUTE_INFO_HIGH_PREF: RoutePreference = RoutePreference::High;
    pub(crate) const ROUTE_INFO_HIGH_PREF_VALID_LIFETIME_SECONDS: u32 = 2_592_000;
    pub(crate) const ROUTE_INFO_HIGH_PREF_PREFIX: Subnet<Ipv6Addr> = unsafe {
        Subnet::new_unchecked(
            Ipv6Addr::new([0x2620, 0x1012, 0x1000, 0x5000, 0xa001, 0x00, 0x00, 0x00]),
            112,
        )
    };

    pub(crate) const ROUTE_INFO_MEDIUM_PREF: RoutePreference = RoutePreference::Medium;
    pub(crate) const ROUTE_INFO_MEDIUM_PREF_VALID_LIFETIME_SECONDS: u32 = 604_800;
    pub(crate) const ROUTE_INFO_MEDIUM_PREF_PREFIX: Subnet<Ipv6Addr> = unsafe {
        Subnet::new_unchecked(
            Ipv6Addr::new([0x2620, 0x1012, 0x1000, 0x0000, 0x00, 0x00, 0x00, 0x00]),
            56,
        )
    };

    pub(crate) const ROUTE_INFO_LOW_PREF: RoutePreference = RoutePreference::Low;
    pub(crate) const ROUTE_INFO_LOW_PREF_VALID_LIFETIME_SECONDS: u32 = 604_800;
    pub(crate) const ROUTE_INFO_LOW_PREF_PREFIX: Subnet<Ipv6Addr> = unsafe {
        Subnet::new_unchecked(Ipv6Addr::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), 0)
    };
}
