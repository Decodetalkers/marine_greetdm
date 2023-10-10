project('marine_greetd', 'rust',
  version: '0.2.0',
  meson_version: '>=0.60')

cargo = find_program('cargo', version: '>= 1.66')

rustc = find_program('rustc', version: '>= 1.66')

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')

build_release = not get_option('debug')

if get_option('marine_cli')
  targetdir = 'debug'
  command = [ cargo , 'build', '--bin', 'marine_cli']
  if build_release
    command += '--release'
    targetdir = 'release'
  endif
  command += [
    '&&',
    'cp',
    meson.global_source_root() / 'target' / targetdir / 'marine_cli',
    '@OUTPUT@'
  ]
  marine_cli_target = custom_target(meson.project_name(),
    output: 'marine_cli',
    build_by_default: true,
    install: true,
    install_dir: bindir,
    console: true,
    command: command
  )
endif
