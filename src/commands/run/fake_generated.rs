macro_rules! setup_faker {
    (
        $($locale:ident),+;
        $(
        mod $mod:ident {
            $($fn:ident = $var:ident);*
            $(;)?
        }
        )*
    ) => {
        use fake::Fake;

        setup_faker! {@consts [$(( $locale ))+]; [] [$($($var ,)*)*]}

        pub fn setup_faker_variables(_sr_template: &::srtemplate::SrTemplate) {
            setup_faker! {@vars [_sr_template] [$(( $locale ))+]; [ $($( $var ,)*)* ]}
        }

        pub fn execute_faker(kind: u16) -> Option<String> {
            setup_faker! (@match [kind] [$(( $locale ))+]; [ $($($mod => $fn = $var;)*)* ])
        }
    };

    ///////////////////////////
    //        CONSTS         //
    ///////////////////////////
    
    (@consts $locales:tt; [$($last:ident)?] []) => { };

    (@consts $locales:tt; [] [$var:ident, $($tail:ident ,)*]) => {
        setup_faker! {@const [0]; $locales; $var}
        setup_faker! {@consts $locales; [$var] [ $($tail ,)* ]}
    };

    (@consts $locales:tt; [$last:expr] [$var:ident, $($tail:ident ,)*]) => {
        setup_faker! {@const [::paste::paste! { [< $last _END >] }]; $locales; $var}
        setup_faker! {@consts $locales; [$var] [ $($tail ,)* ]}
    };

    (@const [$count:expr]; []; $var:ident) => {
        ::paste::paste! { #[allow(dead_code)] const [< $var _END >]: u16 = $count; }
    };

    (@const [$count:expr]; [( $locale:ident ) $($locales:tt)*]; $var:ident) => {
        setup_faker! {@set-const [$locale]; $var = $count;}
        setup_faker! {@const [$count + 1]; [$($locales)*]; $var}
    };

    (@set-const [EN]; $var:ident = $count:expr;) => {
        pub const $var: u16 = $count;
    };

    (@set-const [$locale:ident]; $var:ident = $count:expr;) => {
        ::paste::paste! { pub const [< $var _ $locale>]: u16 = $count; }
    };

    ///////////////////////////
    //         VARS          //
    ///////////////////////////
    (@vars [$sr_template:ident] [ $(( $locale:ident ))* ]; $vars:tt) => {
        $( setup_faker! {@vars [$sr_template] $locale; $vars} )*
    };

    (@vars [$sr_template:ident] $locale:ident; [$($var:ident ,)*]) => {
        $( setup_faker! {@set-var [$sr_template] [$locale]; $var} )*
    };

    (@set-var [$sr_template:ident] [EN]; $var:ident) => {
        $sr_template.add_variable(stringify!($var), & $var);
    };

    (@set-var [$sr_template:ident] [$locale:ident]; $var:ident) => { ::paste::paste! { 
        $sr_template.add_variable(stringify!([< $var _ $locale >]), & [< $var _ $locale >]); 
    } };

    ///////////////////////////
    //        MATCHS         //
    ///////////////////////////
    (@match 
        [$kind:ident]
        $locales:tt;
        [ $($mod:ident => $fn:ident = $var:ident;)* ]
        // $mods:tt
    ) => {
        match $kind {
            $(_ 
                if 
                    $kind >= setup_faker!(@match-key-start $locales; $var) && 
                    $kind <= setup_faker!(@match-key-end $locales; $var) 
                => setup_faker! (@match-value [$kind] $locales; $mod => $fn = $var),
                // => ,
            )*
            _ => None
        }
    };

    (@match-value [$kind:ident] [$(( $locale:ident ))+]; $mod:ident => $fn:ident = $var:ident) => {
        match $kind {
            $(setup_faker!(@match-key [$locale]; $var) => Some(::fake::faker::$mod::raw::$fn(::fake::locales::$locale).fake()),)+
            _ => unsafe { ::core::hint::unreachable_unchecked() }
        }
    };

    (@match-key [EN]; $var:ident ) => {
        $var
    };

    (@match-key [$locale:ident]; $var:ident ) => {
        ::paste::paste!([<$var _ $locale>])
    };

    (@match-key-start [( $locale:ident ) $($locales_tail:tt)+]; $var:ident ) => {
        setup_faker!(@match-key [$locale]; $var)
    };

    (@match-key-end [( $locale:ident )]; $var:ident ) => {
        setup_faker!(@match-key [$locale]; $var)
    };

    (@match-key-end [( $locale:ident ) $($locales_tail:tt)+]; $var:ident ) => {
        setup_faker!(@match-key-end [$($locales_tail)+]; $var)
    };
}

// TODO(Brayan-724): Accept arguments in kinds.
// This can be made creating functions instead of numbers
// or by accepting more parameters in `fake` function

setup_faker! {
    // Locales
    EN,
    FR_FR,
    ZH_TW,
    ZH_CN,
    JA_JP,
    AR_SA,
    PT_BR,
    DE_DE;
    
    mod address {
        CityPrefix = CITY_PREFIX;
        CitySuffix = CITY_SUFFIX;
        CityName = CITY_NAME;
        CountryName = COUNTRY_NAME;
        CountryCode = COUNTRY_CODE;
        StreetSuffix = STREET_SUFFIX;
        StreetName = STREET_NAME;
        TimeZone = TIME_ZONE;
        StateName = STATE_NAME;
        StateAbbr = STATE_ABBR;
        SecondaryAddressType = SECONDARY_ADDRESS_TYPE;
        SecondaryAddress = SECONDARY_ADDRESS;
        ZipCode = ZIP_CODE;
        PostCode = POST_CODE;
        BuildingNumber = BUILDING_NUMBER;
        Latitude = LATITUDE;
        Longitude = LONGITUDE;
        // Geohash(precision: u8) = GEOHASH;
    }

    mod barcode {
        Isbn = ISBN;
        Isbn10 = ISBN10;
        Isbn13 = ISBN13;
    }

    mod boolean {
        // Boolean(ratio: u8) = BOOLEAN;
    }

    mod creditcard {
        CreditCardNumber = CREDIT_CARD_NUMBER;
    }

    mod company {
        CompanySuffix = COMPANY_SUFFIX;
        CompanyName = COMPANY_NAME;
        Buzzword = BUZZWORD;
        BuzzwordMiddle = BUZZWORD_MIDDLE;
        BuzzwordTail = BUZZWORD_TAIL;
        CatchPhrase = CATCH_PHRASE;
        BsVerb = BS_VERB;
        BsAdj = BS_ADJ;
        BsNoun = BS_NOUN;
        Bs = BS;
        Profession = PROFESSION;
        Industry = INDUSTRY;
    }

    mod internet {
        FreeEmailProvider = FREE_EMAIL_PROVIDER;
        DomainSuffix = DOMAIN_SUFFIX;
        FreeEmail = FREE_EMAIL;
        SafeEmail = SAFE_EMAIL;
        Username = USERNAME;
        // Password(len_range: std::ops::Range<usize>) = PASSWORD;
        IPv4 = IPV4;
        IPv6 = IPV6;
        IP = IP;
        MACAddress = MAC_ADDRESS;
        UserAgent = USER_AGENT;
    }

    mod job {
        Seniority = JOB_SENIORITY;
        Field = JOB_FIELD;
        Position = JOB_POSITION;
        Title = JOB_TITLE;
    }

    mod lorem {
        Word = WORD;
        // Words(count: std::ops::Range<usize>) = WORDS;
        // Sentence(count: std::ops::Range<usize>) = SENTENCE;
        // Sentences(count: std::ops::Range<usize>) = SENTENCES;
        // Paragraph(count: std::ops::Range<usize>) = PARAGRAPH;
        // Paragraphs(count: std::ops::Range<usize>) = PARAGRAPHS;
    }

    mod name {
        FirstName = FIRST_NAME;
        LastName = LAST_NAME;
        Title = NAME_TITLE;
        Suffix = NAME_SUFFIX;
        Name = NAME;
        NameWithTitle = NAME_WITH_TITLE;
    }

    mod number {
        Digit = DIGIT;
        // NumberWithFormat<'a>(fmt: &'a str) = NUMBER_WITH_FORMAT;
    }

    mod phone_number {
        PhoneNumber = PHONE_NUMBER;
        CellNumber = CELL_NUMBER;
    }

    mod filesystem {
        FilePath = FILE_PATH;
        FileName = FILE_NAME;
        FileExtension = FILE_EXTENSION;
        DirPath = DIR_PATH;
        MimeType = MIME_TYPE;
        Semver = SEMVER;
        SemverStable = SEMVER_STABLE;
        SemverUnstable = SEMVER_UNSTABLE;
    }

    mod currency {
        CurrencyCode = CURRENCY_CODE;
        CurrencyName = CURRENCY_NAME;
        CurrencySymbol = CURRENCY_SYMBOL;
    }

    mod finance {
        Bic = BIC;
        Isin = ISIN;
    }

    // mod administrative {
    //     HealthInsuranceCode = HEALTH_INSURANCE_CODE;
    // }

    // mod automotive {
    //     LicencePlate = LICENCE_PLATE;
    // }
}
