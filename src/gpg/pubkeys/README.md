# pgpubkeys

Retrieve pgp public keys from various identity platforms.

Public keys half of a public/private key pair.

methods:
* __keybase.io (keybase:)__
* __file (file:)__
* __http (http:)__
* __https (https:)__
* __fingerprint (fp:)__ -- note that short and long "ids" are not permitted due to concerns of spoofability. The full public key cannot be created from the fingerprint, but can be used to ensure that retrieved keys match the desired key exactly.
* __github.com (github:)__ *planned for a later release*
* __hkp (hkp:)__ *planned for a later release*
* __If there is another method that you would like to see here, please file an Issue/Pull Request__


Note: It is highly recommended to include a fingerprint for long-lived keys.

example:
`github:lutostag`
`github:lutostag+fp:9E99626248C8EF631884F5073FE74865735628AC`
