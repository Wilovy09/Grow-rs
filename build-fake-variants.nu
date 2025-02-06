#!/usr/bin/env nu

def "main" [--write] {
  let fake_version = "3.1.0"

  let file = open src/commands/run/fake_generated.rs

  let normalized_content = ($file 
    | lines 
    # Get the setup normalized
    | skip until {|it| $it == '/*=START-SETUP=*/'} 
    | range 2..
    | each {|it| $it | str trim}
    # Remove comments
    | filter {|it| $it != "" and not ($it | str starts-with "//")}
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
    | range ($locales | length)..
    # Split them by modules
    | str join " "
    | split row 'mod'
    # Extract content of each module
    | each {|it| $it | parse --regex ' (?P<mod>.*) { (?P<content>.*) }' }
    # Normalize modules to one table ([mod content])
    | filter {|it| ($it | is-not-empty)}
    | reduce {|it| append $it}
    # Normalize content to one table ([fn var])
    | update content {|it| $it.content | str trim | split row ';' }
    | update content {|it| $it.content | parse --regex '\s?(?P<fn>.*) = (?P<var>.*)' }
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
- `FIRST_NAME` by default the locale is English"

  # Add fakers
  $output += "\n\n## Fakers\n"
  $output += ($modules 
    | sort-by mod
    | each {|it|
      let mod = $it.mod
      let content = $it.content
        | each {|$it| 
          $"  - [`($it.var)`]\(https://docs.rs/fake/($fake_version)/fake/faker/($mod)/raw/struct.($it.fn).html\)"
        }
        | str join "\n"
      $"- [`($mod)`]\(https://docs.rs/fake/($fake_version)/fake/faker/($mod)/raw/index.html\)\n($content)"
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
