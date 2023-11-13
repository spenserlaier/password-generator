# password-generator
A rust command line utility for randomly generating passwords.
## Options
`--help`: print the available command line options"

`--minlength <int>` : min password length"

`--include_nums <bool` : include numbers in password"

`--include_spec <bool>` : include special characters in password"

`--include_ucase <bool>` : include uppercase characters in password"

`--use_dict_words <bool>` : use dictionary words instead of random lowercase alphabetic characters

`--profile <string>` : name of the profile to use. Will pull from local database if such a profile exists

`--new_profile <string>` store a new profile with the provided settings using the given name

`--overwrite <bool>` : if using a profile, overwrite its current settings with the other command line options

`--list_profiles` : prints a list of available profiles

`--profile_info` <string> : prints the provided profile's settings, if the profile exists.


