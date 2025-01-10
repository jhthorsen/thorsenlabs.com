Quelea
======

This project is shut down.

Remaining resources
-------------------

* [Online DOCSIS config file generator](/docsisious/)
* [DOCSIS::ConfigFile Perl module](https://metacpan.org/release/DOCSIS-ConfigFile)
* [Mojo::SNMP Perl module](https://metacpan.org/release/Mojo-SNMP)
* [Net::ISC::DHCPd Perl module](https://metacpan.org/release/Net-ISC-DHCPd)

Project description
-------------------

The idea was a system which could monitor and provision a variety of network equipment.  
The primary target was cable modems and CMTS, but to do that the project had to contain  
more functionality

### A DHCP server

The cable modem and customer computer equipment need IP addresses. I decided to use  
[ISC DHCPd](https://www.isc.org/software/dhcp) for this purpose, because  
it can dynamically serve config files based on MAC address (option 82).  
In addition, it's a very stable and capable DHCP server for all other equipment,  
beside cable modems.

### A TFTP server

After a cable modem has received a response from the DHCP server, it will try to  
download a config file from a TFTP server. I decided to write this TFTP server in  
pure perl, because I could then build the config files dynamically from profiles  
stored in a backend. The profiles would be split into different segments, which  
mostly should default to a "most common" profile and then the bandwidth profile  
would be applied on top of that.

I started out with [Net::TFTPd](https://metacpan.org/module/Net::TFTPd)  
but realized that it was not easy to hook into and it wasn't all that effective.  
I then decided to write my own implementation called [POE::Component::TFTPd](https://metacpan.org/module/POE::Component::TFTPd),  
which I was quite pleased with. Even so it tried to make it even faster, which  
resulted in [AnyEvent::TFTPd](https://metacpan.org/module/AnyEvent::TFTPd),  
but the module was shut down by the project owner of AnyEvent. If I was continuing  
the development, I would probably write a Mojo::IOLoop based version instead, since  
Mojolicious is a fantastic framework.

### A collector daemon

I wanted the system to be plug and play, so the collector daemon would start out  
by probing the computers in the same network, checking if they could respond  
on SNMP requests. If they could they would automatically be added to the Quelea  
frontend, where the credentials (if any) would have to be added before the  
collector again started getting information from the various equipment. The  
collector would in the first place be "limited" to only supporting SNMP.

The most important data (imo) to collect would be:

* CMTS
  * Tx on downstream
  * SNR on upstream
  * Maybe cable modem signals.
* Cable modem
  * Tx on upstream
  * SNR on downstream
  * Rx on downstream
  * Micro reflections on downstream
