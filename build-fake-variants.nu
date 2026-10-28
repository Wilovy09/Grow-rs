#!/usr/bin/env nu

def "main" [--write] {
  let fake_version = "3.1.0"

  let file = open src/commands/run/fake_generated.rs

  let normalized_content = ($file 
    | lines 
    # Get the setup normalized
    | skip until {|it| $it == '/*=START-SETUP=*/'} 
    | slice 2..
    | each {|it| $it | str trim}
    # Remove comments
    | where $it != "" and not ($it | str starts-with "//")
  )

  # Get locales
  let locales = ($normalized_content
    # The locale separator
    | take until {|it| $it | str ends-with ';'}
    # Remove commas at the end of each locale
    | each {|it| $it | str substring ..-2}
    # Normalize to one table ([name code])
    | each {|it| $it | parse '"{name}" = {code}'}
    | reduce {|it| append $it}
  )

  let modules = ($normalized_content
    | slice ($locales | length)..
    # Split them by modules
    | str join " "
    | split row 'mod'
    # Extract content of each module
    | each {|it| $it | parse --regex ' (?P<mod>.*) { (?P<content>.*) }' }
    # Normalize modules to one table ([mod content])
    | where ($it | is-not-empty)
    | reduce {|it| append $it}
    # Normalize content to one table ([fn var])
    | update content {|it| $it.content | str trim | split row ';' }
    | update content {|it| $it.content | parse --regex '\s?(?P<dummy>\(dummy\) )?(?P<fn>.*) = (?P<var>.*)' | update dummy {$in == "(dummy) "} }
  ) 

  mut output = $"# Fake variants
Using [fake v($fake_version)]\(https://github.com/cksac/fake-rs) for generating fake data in different languages. Currently supports:
"

  # Add locales
  $output += "\n"
  $output += "| Language | Code |\n|--|--|\n"
  $output += ($locales | each {|it| $"| ($it.name) | ($it.code) |"} | str join "\n")
  $output += "\n"
  $output += "\n"
  $output += "You can add any code at the end of any faker to convert it to that locale.\n"
  $output += "Examples: 
- `FIRST_NAME_ZH_TW` for first name in Chinese
- `FIRST_NAME` by default the locale is English

## Usage with Inline Attributes (New Syntax)

The new inline attributes syntax makes it easier to organize your fake data seeders:

```ron
{
    // Modern syntax with inline attributes
    #[repeat = 50]
    #[schema = \"users\"]
    User: {
        \"first_name\": \"{fake(FIRST_NAME)}\",
        \"last_name\": \"{fake(LAST_NAME)}\",
        \"email\": \"{fake(SAFE_EMAIL)}\",
        \"phone\": \"{fake(PHONE_NUMBER)}\",
        \"address\": \"{fake(STREET_ADDRESS)}\",
        \"city\": \"{fake(CITY)}\",
        \"created_at\": \"2024-01-01 00:00:{i}\",
    },

    // Multiline attributes for better readability
    #[repeat = 20]
    #[schema = \"products\"]
    products: {
        \"name\": \"{fake(WORD)}\",
        \"description\": \"{fake(SENTENCE_4_TO_10)}\",
        \"price\": \"{fake(NUMBER_1_TO_1000)}\",
        \"sku\": \"SKU-{i}\",
    },

    // Mix locales
    #[repeat = 10] customers: {
        \"name_en\": \"{fake(FIRST_NAME)}\",
        \"name_ja\": \"{fake(FIRST_NAME_JA_JP)}\",
        \"city_en\": \"{fake(CITY)}\",
        \"city_de\": \"{fake(CITY_DE_DE)}\",
    },
}
```

## Legacy Syntax (Still Supported)

```ron
{
    User(50): {
        \"first_name\": \"{fake(FIRST_NAME)}\",
        \"email\": \"{fake(SAFE_EMAIL)}\",
    },
}
```
"

  # Add fakers
  let prefix_url = $"https://docs.rs/fake/($fake_version)/fake";
  $output += "\n## Fakers\n"
  $output += ($modules 
    | sort-by mod
    | each {|it|
      let mod = $it.mod
      let children_dummy = $it.content | any {$in.dummy};

      let content = $it.content
        | each {|$it| 
          let url = if $it.dummy {
            $"($prefix_url)/($mod)/struct.($it.fn).html"
          } else {
            $"($prefix_url)/faker/($mod)/raw/struct.($it.fn).html"
          };
          $"  - [`($it.var)`]\(($url)\)"
        }
        | str join "\n"

      let url = if $children_dummy {
        $"($prefix_url)/($mod)/index.html"
      } else {
        $"($prefix_url)/faker/($mod)/raw/index.html"
      };

      $"- [`($mod)`]\(($url)\)\n($content)"
    }
    | str join "\n"
  )

  if $write {
    $output | save -f ./FAKE-VARIANTS.md
  } else {
    $output | print

    print -e $"
(ansi cyan_bold)[INFO](ansi --escape '21m') Running as dry by default.
(ansi cyan_bold)[INFO](ansi --escape '21m') Use `--write` to replace `FAKE-VARIANT.md` file.
(ansi reset)"
  }
}
