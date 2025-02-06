# Fake variants
Using [fake v3.1.0](https://github.com/cksac/fake-rs) for generating fake data in different languages. Currently supports:

| Language | Code |
|--|--|
| English | EN |
| Arabic | AR_SA |
| German | DE_DE |
| French | FR_FR |
| Japanese | JA_JP |
| Protugese Brazilian | PT_BR |
| Traditional Chinese | ZH_TW |
| Simplified Chinese | ZH_CN |

You can add any code at the end of any faker to convert it to that locale.
Examples: 
- `FIRST_NAME_ZH_TW` for first name in Chinese
- `FIRST_NAME` by default the locale is English

## Fakers
- [`address`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/index.html)
  - [`CITY_PREFIX`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.CityPrefix.html)
  - [`CITY_SUFFIX`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.CitySuffix.html)
  - [`CITY_NAME`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.CityName.html)
  - [`COUNTRY_NAME`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.CountryName.html)
  - [`COUNTRY_CODE`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.CountryCode.html)
  - [`STREET_SUFFIX`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.StreetSuffix.html)
  - [`STREET_NAME`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.StreetName.html)
  - [`TIME_ZONE`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.TimeZone.html)
  - [`STATE_NAME`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.StateName.html)
  - [`STATE_ABBR`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.StateAbbr.html)
  - [`SECONDARY_ADDRESS_TYPE`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.SecondaryAddressType.html)
  - [`SECONDARY_ADDRESS`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.SecondaryAddress.html)
  - [`ZIP_CODE`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.ZipCode.html)
  - [`POST_CODE`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.PostCode.html)
  - [`BUILDING_NUMBER`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.BuildingNumber.html)
  - [`LATITUDE`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.Latitude.html)
  - [`LONGITUDE`](https://docs.rs/fake/3.1.0/fake/faker/address/raw/struct.Longitude.html)
- [`barcode`](https://docs.rs/fake/3.1.0/fake/faker/barcode/raw/index.html)
  - [`ISBN`](https://docs.rs/fake/3.1.0/fake/faker/barcode/raw/struct.Isbn.html)
  - [`ISBN10`](https://docs.rs/fake/3.1.0/fake/faker/barcode/raw/struct.Isbn10.html)
  - [`ISBN13`](https://docs.rs/fake/3.1.0/fake/faker/barcode/raw/struct.Isbn13.html)
- [`company`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/index.html)
  - [`COMPANY_SUFFIX`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.CompanySuffix.html)
  - [`COMPANY_NAME`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.CompanyName.html)
  - [`BUZZWORD`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.Buzzword.html)
  - [`BUZZWORD_MIDDLE`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.BuzzwordMiddle.html)
  - [`BUZZWORD_TAIL`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.BuzzwordTail.html)
  - [`CATCH_PHRASE`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.CatchPhrase.html)
  - [`BS_VERB`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.BsVerb.html)
  - [`BS_ADJ`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.BsAdj.html)
  - [`BS_NOUN`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.BsNoun.html)
  - [`BS`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.Bs.html)
  - [`PROFESSION`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.Profession.html)
  - [`INDUSTRY`](https://docs.rs/fake/3.1.0/fake/faker/company/raw/struct.Industry.html)
- [`creditcard`](https://docs.rs/fake/3.1.0/fake/faker/creditcard/raw/index.html)
  - [`CREDIT_CARD_NUMBER`](https://docs.rs/fake/3.1.0/fake/faker/creditcard/raw/struct.CreditCardNumber.html)
- [`currency`](https://docs.rs/fake/3.1.0/fake/faker/currency/raw/index.html)
  - [`CURRENCY_CODE`](https://docs.rs/fake/3.1.0/fake/faker/currency/raw/struct.CurrencyCode.html)
  - [`CURRENCY_NAME`](https://docs.rs/fake/3.1.0/fake/faker/currency/raw/struct.CurrencyName.html)
  - [`CURRENCY_SYMBOL`](https://docs.rs/fake/3.1.0/fake/faker/currency/raw/struct.CurrencySymbol.html)
- [`filesystem`](https://docs.rs/fake/3.1.0/fake/faker/filesystem/raw/index.html)
  - [`FILE_PATH`](https://docs.rs/fake/3.1.0/fake/faker/filesystem/raw/struct.FilePath.html)
  - [`FILE_NAME`](https://docs.rs/fake/3.1.0/fake/faker/filesystem/raw/struct.FileName.html)
  - [`FILE_EXTENSION`](https://docs.rs/fake/3.1.0/fake/faker/filesystem/raw/struct.FileExtension.html)
  - [`DIR_PATH`](https://docs.rs/fake/3.1.0/fake/faker/filesystem/raw/struct.DirPath.html)
  - [`MIME_TYPE`](https://docs.rs/fake/3.1.0/fake/faker/filesystem/raw/struct.MimeType.html)
  - [`SEMVER`](https://docs.rs/fake/3.1.0/fake/faker/filesystem/raw/struct.Semver.html)
  - [`SEMVER_STABLE`](https://docs.rs/fake/3.1.0/fake/faker/filesystem/raw/struct.SemverStable.html)
  - [`SEMVER_UNSTABLE`](https://docs.rs/fake/3.1.0/fake/faker/filesystem/raw/struct.SemverUnstable.html)
- [`finance`](https://docs.rs/fake/3.1.0/fake/faker/finance/raw/index.html)
  - [`BIC`](https://docs.rs/fake/3.1.0/fake/faker/finance/raw/struct.Bic.html)
  - [`ISIN`](https://docs.rs/fake/3.1.0/fake/faker/finance/raw/struct.Isin.html)
- [`internet`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/index.html)
  - [`FREE_EMAIL_PROVIDER`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.FreeEmailProvider.html)
  - [`DOMAIN_SUFFIX`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.DomainSuffix.html)
  - [`FREE_EMAIL`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.FreeEmail.html)
  - [`SAFE_EMAIL`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.SafeEmail.html)
  - [`USERNAME`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.Username.html)
  - [`IPV4`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.IPv4.html)
  - [`IPV6`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.IPv6.html)
  - [`IP`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.IP.html)
  - [`MAC_ADDRESS`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.MACAddress.html)
  - [`USER_AGENT`](https://docs.rs/fake/3.1.0/fake/faker/internet/raw/struct.UserAgent.html)
- [`job`](https://docs.rs/fake/3.1.0/fake/faker/job/raw/index.html)
  - [`JOB_SENIORITY`](https://docs.rs/fake/3.1.0/fake/faker/job/raw/struct.Seniority.html)
  - [`JOB_FIELD`](https://docs.rs/fake/3.1.0/fake/faker/job/raw/struct.Field.html)
  - [`JOB_POSITION`](https://docs.rs/fake/3.1.0/fake/faker/job/raw/struct.Position.html)
  - [`JOB_TITLE`](https://docs.rs/fake/3.1.0/fake/faker/job/raw/struct.Title.html)
- [`lorem`](https://docs.rs/fake/3.1.0/fake/faker/lorem/raw/index.html)
  - [`WORD`](https://docs.rs/fake/3.1.0/fake/faker/lorem/raw/struct.Word.html)
- [`name`](https://docs.rs/fake/3.1.0/fake/faker/name/raw/index.html)
  - [`FIRST_NAME`](https://docs.rs/fake/3.1.0/fake/faker/name/raw/struct.FirstName.html)
  - [`LAST_NAME`](https://docs.rs/fake/3.1.0/fake/faker/name/raw/struct.LastName.html)
  - [`NAME_TITLE`](https://docs.rs/fake/3.1.0/fake/faker/name/raw/struct.Title.html)
  - [`NAME_SUFFIX`](https://docs.rs/fake/3.1.0/fake/faker/name/raw/struct.Suffix.html)
  - [`NAME`](https://docs.rs/fake/3.1.0/fake/faker/name/raw/struct.Name.html)
  - [`NAME_WITH_TITLE`](https://docs.rs/fake/3.1.0/fake/faker/name/raw/struct.NameWithTitle.html)
- [`number`](https://docs.rs/fake/3.1.0/fake/faker/number/raw/index.html)
  - [`DIGIT`](https://docs.rs/fake/3.1.0/fake/faker/number/raw/struct.Digit.html)
- [`phone_number`](https://docs.rs/fake/3.1.0/fake/faker/phone_number/raw/index.html)
  - [`PHONE_NUMBER`](https://docs.rs/fake/3.1.0/fake/faker/phone_number/raw/struct.PhoneNumber.html)
  - [`CELL_NUMBER`](https://docs.rs/fake/3.1.0/fake/faker/phone_number/raw/struct.CellNumber.html)