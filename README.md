# NAME
**ssh-keyget-rs** - get SSH public key from server

# SYNOPSIS
**ssh-keyget-rs**
[*host:port*]
[*key_type(ed25519,rsa_sha2,ecdsa,rsa)*]

# DESCRIPTION
**ssh-keyget-rs**
is a utility for getting the SSH public key from a server.

**ssh-keyget-rs**
does not need login access to the server.

The options are as follows:

*key_type*

> Specify the type of the key to fetch from the server.
> The possible values are
> "ed25519",
> "rsa_sha2",
> "ecdsa",
> "rsa". 


If a public key obtained using
**ssh-keyget-rs**
is used without verifying the key, users will be vulnerable to
*man in the middle*
attacks.

# FILES

*None*

# EXAMPLES

Print the RSA public key for server
*hostname*:

	ssh-keyget-rs hostname:port rsa

Save RSA public key for server
*hostname*:

	ssh-keyget-rs hostname:port rsa > publickey

# SEE ALSO

ssh(1),
sshd(8)

# AUTHORS

sjp27 &lt; https://github.com/sjp27 &gt;
implemented ssh-keyget-rs.
