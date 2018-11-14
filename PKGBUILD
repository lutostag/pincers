# Maintainer: Greg Lutostanski <greg.lutostanski@mobilityhouse.com>
pkgname=pincers
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="A more secure way to run scripts from the web"
license=('MIT')
url=https://github.com/lutostag/pincers

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git="$source"
}
