dnl Copyright (c) 2018, Suphanat Chunhapanya
dnl This file is part of Kelner.
dnl
dnl Kelner is free software; you can redistribute it and/or
dnl modify it under the terms of the GNU General Public License
dnl as published by the Free Software Foundation; either version 2
dnl of the License, or (at your option) any later version.
dnl
dnl Kelner is distributed in the hope that it will be useful,
dnl but WITHOUT ANY WARRANTY; without even the implied warranty of
dnl MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
dnl GNU General Public License for more details.
dnl
dnl You should have received a copy of the GNU General Public License
dnl along with Kelner.  If not, see <https://www.gnu.org/licenses/>.

AC_PREREQ([2.69])
AC_INIT([kelner], [0.0.1], [haxx.pop@gmail.com])

AC_ARG_ENABLE(fatal-warnings,
     AS_HELP_STRING(--enable-fatal-warnings,
                    [tell the compiler to treat all warnings as errors.]))

KELNER_AUTHOR_NAME="Suphanat Chunhapanya"
AC_SUBST(KELNER_AUTHOR_NAME)

KELNER_AUTHOR_EMAIL="haxx.pop@gmail.com"
AC_SUBST(KELNER_AUTHOR_EMAIL)

KELNER_LOADING_ADDRESS="0x100000"
AC_SUBST(KELNER_LOADING_ADDRESS)

AC_CONFIG_FILES([
  Makefile
  kernel/layout.ld
  kernel/Cargo.toml
])

KELNER_CONFIG_FILES=$ac_config_files
AC_SUBST(KELNER_CONFIG_FILES)

AC_PROG_MAKE_SET
AC_OUTPUT
