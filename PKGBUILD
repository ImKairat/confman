# PKGBUILD
pkgname=confman
pkgver=0.1.0
pkgrel=1
pkgdesc="confman is a lightweight, command-line configuration manager for Linux systems."
arch=('x86_64')
url="https://github.com/ImKairat/confman"
license=('GPL3')
depends=()
makedepends=('rust' 'cargo' 'git')
source=("git+$url.git")
md5sums=('SKIP')

build() {
  cd "$srcdir/$pkgname"
  cargo build --release
}

package() {
  install -Dm755 "$srcdir/$pkgname/target/release/confman" "$pkgdir/usr/bin/confman"
}
