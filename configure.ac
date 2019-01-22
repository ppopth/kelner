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

AC_ARG_ENABLE(verbose,
     AS_HELP_STRING(--enable-verbose,
                    [show the verbose for all operations.]))

KELNER_AUTHOR_NAME="Suphanat Chunhapanya"
AC_SUBST(KELNER_AUTHOR_NAME)

KELNER_AUTHOR_EMAIL="haxx.pop@gmail.com"
AC_SUBST(KELNER_AUTHOR_EMAIL)

KELNER_ARCH="x86_64"
AC_SUBST(KELNER_ARCH)

KELNER_KERNEL_CODE_START="0x100000"
AC_SUBST(KELNER_KERNEL_CODE_START)

KELNER_KERNEL_CODE_END="0x700000"
AC_SUBST(KELNER_KERNEL_CODE_END)

KELNER_SAMPLE_ELF_START="0x700000"
AC_SUBST(KELNER_SAMPLE_ELF_START)

KELNER_SAMPLE_ELF_END="0x800000"
AC_SUBST(KELNER_SAMPLE_ELF_END)

KELNER_KERNEL_HEAP_START="0x800000"
AC_SUBST(KELNER_KERNEL_HEAP_START)

KELNER_KERNEL_HEAP_END="0x1000000"
AC_SUBST(KELNER_KERNEL_HEAP_END)

KELNER_KERNEL_STACK_START="0x1000000"
AC_SUBST(KELNER_KERNEL_STACK_START)

KELNER_KERNEL_STACK_END="0x4000000"
AC_SUBST(KELNER_KERNEL_STACK_END)

KELNER_USED_KERNEL_MEMORY="\
${KELNER_KERNEL_CODE_START}-${KELNER_KERNEL_CODE_END},\
${KELNER_SAMPLE_ELF_START}-${KELNER_SAMPLE_ELF_END},\
${KELNER_KERNEL_HEAP_START}-${KELNER_KERNEL_HEAP_END},\
${KELNER_KERNEL_STACK_START}-${KELNER_KERNEL_STACK_END}\
"
AC_SUBST(KELNER_USED_KERNEL_MEMORY)

KELNER_IDENTITY_MAP_MEMORY="\
0x0-0x100000,\
${KELNER_USED_KERNEL_MEMORY}\
"
AC_SUBST(KELNER_IDENTITY_MAP_MEMORY)

dnl Other parameters may be changed, but this one must not! It's not easy
dnl to change a page size arbitrarily.
KELNER_PAGE_SIZE="0x1000"
AC_SUBST(KELNER_PAGE_SIZE)

dnl Other parameters may be changed, but this one must not! It's not easy
dnl to change a page size arbitrarily.
KELNER_PAGE_SIZE_LOG="12"
AC_SUBST(KELNER_PAGE_SIZE_LOG)

dnl Other parameters may be changed, but this one must not! It's not easy
dnl to change a page size arbitrarily.
KELNER_BOOTLOADER_PAGE_SIZE="0x200000"
AC_SUBST(KELNER_BOOTLOADER_PAGE_SIZE)

dnl Other parameters may be changed, but this one must not! It's not easy
dnl to change a page size arbitrarily.
KELNER_BOOTLOADER_PAGE_SIZE_LOG="21"
AC_SUBST(KELNER_BOOTLOADER_PAGE_SIZE_LOG)

AC_CONFIG_FILES([
  .gdbinit
  Makefile
  bootloader/config.asm
  kernel/layout.ld
  kernel/Cargo.toml
  kernel/src/config.rs
])

if test "x$enable_fatal_warnings" = "xyes"; then
  KELNER_FATAL_WARNINGS=
else
  # The sharp is used to comment the code in Makefile
  KELNER_FATAL_WARNINGS=#
fi
AC_SUBST(KELNER_FATAL_WARNINGS)

if test "x$enable_verbose" = "xyes"; then
  KELNER_VERBOSE=
else
  # The sharp is used to comment the code in Makefile
  KELNER_VERBOSE=#
fi
AC_SUBST(KELNER_VERBOSE)

KELNER_CONFIG_FILES=$ac_config_files
AC_SUBST(KELNER_CONFIG_FILES)

AC_PROG_MAKE_SET
AC_OUTPUT
