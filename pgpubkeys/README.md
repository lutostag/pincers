# pgpubkeys

Retrieve pgp public keys from various identity platforms.

Public keys half of a public/private key pair.

methods:
* __keybase.io (kb:)__
* __github.com (gh:)__
* __hkp (hkp:)__
* __file (file:)__
* __https (https:)__
* __fingerprint (fp:)__ -- note that short and long "ids" are not permitted due to concerns of spoofability. The full public key cannot be created from the fingerprint, but can be used to ensure that retrieved keys match the desired key exactly.
* __If there is another method that you would like to see here, please file an Issue/Pull Request__


Note: It is highly recommended to include a fingerprint for long-lived keys.

example:
`gh:lutostag`
`gh:lutostag+fp:9E99626248C8EF631884F5073FE74865735628AC`
