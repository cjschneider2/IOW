# IOW
`In Other Words` is a BSD-WTF clone written in rust

# Usage

The basic usage is `iow <acronym>` which will print out the (hopefully generally
accepted) meaning of the acronym.

# Example

    $> iow lol
    $> lol: Laughing out loud

# Extra Dictionaries

`iow` will try automatically load dictionaries from the `IOW_DICT` environment
variable with the extention `*.iow`. When this variable is not set the program
will look in the home directory of the current user in the folder `.iow/` for 
the dictionary files.
