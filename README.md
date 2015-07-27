# Asciiauthor

asciiauthor is a project-generator for asciidoc projects.

## Requirements

Asciiauthor itself is written in rust (1.1). To compile it yourself, you need rust installed.

```bash
git clone https://github.com/Richterrettich/asciiauthor.git
cd asciiauthor
cargo build --release
```
Asciiauthor on its own is pretty boring. You'll need assciidoctor as well.
Follow the instructions from http://asciidoctor.org/


## Usage

First, initialize a new book-project:

```bash
asciiauthor init my_awesome_project
# |
# |-my_awesome_project
#        |- content
#             |- index.adoc
#             |- images
#        |- includes
#        |- .git
#        |- .gitignore
```

If you haven't setup your email/name in a global git config, asciiauthor will ask you
to enter these information.

Now move towards the content directory. You can see one directory for images, as
a file called `index.adoc`. This file is the root-asciidoc file. You can add your content there.

### section command

By the time your book grows, a one-file approach will become unmanageable. Asciiauthor
is build to support modular book writing. You can use the `section` command to create
a new section within your book:

```bash
cd my_awesome_project/content
# ls: index.adoc   images
asciiauthor section introduction
# |
# |- index.adox
# |- images
# |- 1_introduction
#        |- index.adoc
#        |- images
#
```

The newly created section will be added within your root-index:

```bash
cat index.adox
## output
#= my_awesome_project
#Rene Richter <awesome_email@gmail.com>
#include::../includes/config.adoc[]
#
#toc::[]
#
#include::1_introduction/index.adoc[]
```

You can nest these sections as deep as you like. But be careful, if you overdo it,
your project will become a mess.

You can't use the `section` command outside of your content directory (or any subdirectory)

Some words of warning: Do note rename your directories manually. This will cause
havoc.



### Managing sections

Once a section is created, you can move it arround or delete it with asciiauthor.
Consider the following:

```bash
# |
# | - 1_foo
# | - 2_bar
# | - 3_bazz
#       |- 1_foobar
asciiauthor swap 1 3
# |
# | - 1_bazz
#       |- 1_foobar
# | - 2_bar
# | - 3_foo
asciiauthor move 1 3
# |
# | - 1_bar
# | - 2_foo
# | - 3_bazz
#       |- 1_foobar
asciiauthor delete 2
# |
# | - 1_bar
# | - 2_bazz
#       |- 1_foobar
cat index.adoc
#= my_awesome_project
#Rene Richter <awesome_email@gmail.com>
#include::../includes/config.adoc[]
#
#toc::[]
#
#include::1_bar/index.adoc[]
#
#include::2_bazz/index.adoc[]
```

As you can see, these commands will keep your index.adox in sync with your directories.

Delete will delete the section. It will then rearrange the other sections to fill gaps.

There is a subtle difference between swap and move. Swap will just swap the position
of two sections. In this example, it will swap section 1 with section 3.

Move on the other hand will rearrange all sections in between. So if you move
section 1 to location 3, the other sections in between will be reduced by 1.
This works in both directions:

```bash
# |
# | - 1_bazz
#       |- 1_foobar
# | - 2_bar
# | - 3_foo
asciiauthor move 3 1
# |
# | - 1_foo
# | - 2_bazz
#       |- 1_foobar
# | - 3_bar
```

### Further information

asciiauthor uses the filesystem/git to determine your project root. There is no database involved.
The information is stored in the `.git/description` file:
```bash
cd my_awesome_project
cat .git/description
# output
# my_awesome_project_book
 ```


 ## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
