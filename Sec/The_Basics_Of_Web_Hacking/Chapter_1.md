# Chapter 1 - The Basics of Web Hacking

## What is a web application?

In this book the term `web application` will be used throughout the book for any web-based software that performs actions (functionality) based on user input and usually interacts with backend systems.

## What you need to know about web servers?

A `web server` is just a piece of software running on the operation system of a server that allows connetions ot access a web application.

### Windows Server

The most common servers are `Internet Information Services` (IIS) on a `Windows server`.

Default directory: `C:\Inetpub\wwwroot`.

### Linux Server

The most common servers are `Apache Hypertext Transfer Protocol` (HTTP) on a `Linux server`.

Most of the web applications are housed in: `/var/www`.

Relevant files when web hacking:

- `/etc/shadow`: Where the `passwords hashes` for all users of the system reside.
- `/usr/lib`: This directory includes object files and `internal binaries` that are not intended to be executed by users or shell scripts. All dependency data used by the application will also reside in this directory. Although there is nothing executable here, you can really `ruin somebody's day` by deleting all of the dependency files for an application.
- `/var/\*`: This directory includes the files for `databases, system logs, and the source code` for web application itself!.
- `/bin`: This directory contain programs that the system system needs to operate, such as the shells, ls, grep, and other essential and `important binaries`. Most standart operating system commands are located here as separated executable binary files.

## What you need to know about HTTP?

Its a completely plaintext protocol, there is no assumption of security or privacy when using `HTTP`. Every client request and web aplication response is a brand new, independent event without knowledge of any previous requests. However it keep track of the client resquests so you can complete multistep transactions.

You can inspect all the details of HTTP with tools suck as `Wireshark` or any `local HTTP proxy`.

Secure HTTP (`HTTPS`) ensure that `man-in-the-middle` and `eavesdroping` attacks doensn't occur. Encrypted communication.

### HTTP Cycles

One of the most important and fundamental operations of every web application is the cycle of requests made by client's browsers and the responses returned by the server. It's a very simple premise that happens manyu of times every day:

- A browser sends a request filled with parameters (variables) holding `user input`
- The web server sends a `response` that is dictated by the submitted request

The web application may act based on the values of the paramenters, so they are prime targets for hackers to attack with `malicious paramenter` values to exploit the web application and web server.

### Noteworthy HTTP Headers

Each `HTTP cycle` also includes headers in both the client request and the server response that transmit details about the request or response.

Sent to client browser:

- `Set-Cookie`: This header most commonly provides the session identifier (cookie) to the client. If a hacker can steal a user’s session, they can `assume the identity` of the exploited user.

- `Content-Length`: This header’s value is the length of the response body in bytes. You can look for variation in the number of bytes of the response to help `decipher the application’s response` to input. This is especially applicable when conducting `brute force` (repetitive guessing) attacks.

- `Location`: This header is used when an application redirects a user to a new page. It can be used to help `identify pages` that are only allowed after successfully authenticating to the application, for example.

Sent by client browser:

- `Cookie`: This header sends the cookie (or several cookies) back to the server to maintain the user’s session. This cookie header value `should always match the value of the set-cookie` header that was issued by the server. It may provide a valid session with the application that can be used in attacks against other application users.

- `Referrer`: This header lists the webpage that the user was previously on when the next web request was made. Think of this header as storing the “the last page visited.” This value can be easily changed. If the application is relying on this header for any sense of security, it can `easily be bypassed `with a forged value.

### Noteworthy HTTP status code

- 100s: These responses are purely informational from the web server and usually mean that additional responses from the web server are forthcoming.

- 200s: These responses signal the `client’s request was successfully accepted`

- 300s: These responses are used to signal redirection where additional responses will be sent to the client.

- 400s: These responses are used to signal an error in the request from the client.

    - 401 Unauthorized
    - 403 Forbidden
    - 404 Not Found

- 500s: These responses are used to signal an error on the server side.

    - 500 Internal Server Error
    - 503 Service Unavailable

## The Basccs of Web Hacking: Our Approach

1. Reconnaissance
2. Scanning
3. Exploitation
4. Fix

### Our targets

- Web server: Application running on an operation system (Services running on open ports).
    - Objectives: Gain unauthorized access to the web server's `file structure` and `system files`

- Web application: The source code running the web server.
    - Objectives: Perform `unauthorized actions` within the web application

- Web user: the internal users that manage the web applications (admin and users) and external users.
    - Objectives: `XSS` (cross-site scripting) and `CSRF` (cross-site request forgery). Technical socal enginnering attacks that rely on no existing web vulnerabilities are also applicable.

### Our tools

- `Burp Suite`: widely accepted as the #1 web hacking tool collection
- `Zed Attack Proxy (ZAP)`: similar to Burp Suite, but also includes a free vulnerability scanner.
- `Nmap, Nessus and Nikto`: vulnerability scanning
- `Metasploit`: exploitation of the web server
- `sqlmap, John the Ripper and the Social Engineering Toolkit (SET)`:specific role

## Web Apps Touch Every Part of IT

Possible attack vectors:

- Database server and database: the system that is hosting the database that the web application uses may be vulnerable to attacks of (`CRUD`)

- File server: allows file `upload and/or download` may be vulnerable to attacks that allow server resources to be accessed from an unauthorized attacker.

- Third-party: modules of code, content management systems (`CMSs`), because of the widespread adoption and avaliable documentation of these systems.

## Existing Methodologies

The two most widely accepted pen test methodologis today are the [Open Source Security Testing Methodology Manual](http://www.isecom.org/research/osstmm.html) or [Penetration Testing Execution Standard](http://www.pentest-standard.org/)

### The Open-Source Security Testing Methodology Manual ([OSSTM](http://www.isecom.org/research/osstmm.html))

1. Information and data controls
2. Personnel security awareness levels
3. Fraud and social engineering levels
4. Computer and telecommunications networks, wireless devices, and mobile devices
5. Physical security access protocols, security process, and physical locations

The OSSTM measures the technical details of each of these areas and provides guidance on what to do before, during and after a security assessment

### Penetration Testing Execution Standard ([PTES](http://www.pentest-standard.org/))

A new standard aimed at providing common language for all penetration testers and security assessment professionals to follow. PTES provides a client with a baselne of their own security posture, they are in a better position to make sense of penetration testing findings.

### Making Sense Of Existing Methodologies

Both of these standarts cover every possible aspect of security testing, and they do a great job. But for beginning hackers it's sensory overload

What you need is boil down all the great information in standards such the OSSTM and PTES into a more manageable methodology so that beginning hackers aren't overhwelmed. That's the exact goal of this book.

### Most common Web Vulnerabilities

Our targets will all be exploited by attacking well-understood vulnerabilities. Although there are several other web-related vulnerabilities, these are the ones we are going to concentrate on as we work through the chapters.

#### Injection

Occur when a untrusted user data are sent to the web application as part of a `command` or `query`. The attacker's hostile data can trick the web application into executing uninteded commands or accessing unaunthorized data (input accepted by the web application without the appropriate sanitization).

Some of the most common injections attacks targer the following functionality:

- Structured query language (`SQL`) queries
- Lightweight directory access protocol (`LDAP`) queries
- XML path language (`XPATH`) queries
- Operating system (`OS`) commands

#### Cross-Site Scripting (XSS)

Occurs when user input is accepted by the application as part of request and then in used in the output of the response without proper output encoding. XSS alows attackers to execute scripts (JavaScript and VBScript) in the victim's browser, which can hijack user sessions, act as keyloggers, redirect the user to malicious sites, etc...

- Reflected: Is much more widespread in web applications and is considered to be less harmful because it's one-time attack where the payload contains the malicious script (Attack 1:1 hacker and victim).

- Stored: Is harder to find in web applications, but is more damaging because it persists across multiple requests and can exploit numerous users with one attack. Occurs when a hacker is able to inject the malicious script into the aplication and have it be avaliable to all visiting users (Attack 1:n hacker and everyone who enter the website)

#### Broken Authentication and Session Management

Sessions are the unique identifiers that are assigned to users after authenticating and have many vulnerabilities or attacks associated with how these identifiers are used by the web application.

Application functions related to authentication and session management are often not implemented correctly, allowing attackers to compromise passwords, keys, session tokens, or exploit other implementation flaws to assume other user's identities. Other funcionality of the web application that is under the authentication umbrella also includes password reset, password change, account recovery, etc...

#### Cross-Site Request Forgery

CSRF occurs when a hacker is able to send a well-crafted request to an authenticated user that includes the necessary parameters to complete a valid application request without the victim (user) even realizing it.

CSRF may also perform a valid request made to the web application. Some results of CSRF are changing a password, creating a new user, or creating web appication content via CMS.

#### Security Misconfiguration

This category deals with the security of the entire application stack (Operating System, Web Server, Database Management Systems)

Example of vulnerabilities:

- Out-of-date or unnecessary software
- Unnecessary services enabled
- Insecure account policies
- Verbose error messages

Effective security requires having a secure configuration defined and deployed for the application, frameworks, application server, web server, database server, and operating system. All these settings should be defined, implemented, and mantained, as many are not shipped with secure defaults. This includes keeping all software up to date, includeing all code libraries used by the application.

### Setting Up a Test Environment
