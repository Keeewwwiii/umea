
# <img src="src/assets/logo.svg" height="70"> umea
*Universial Multi-platform Engine for Applications*

A lightweight runtime for running web apps on your desktop, built with Rust.

Umea is an attempt to make desktop apps using HTML, CSS and JavaScript without needing to ship a whole copy of Chromium with every app.

It's still a work in progress.
Why?

Electron works, but bundling a full browser with every app can make distributions pretty large. Umea is an attempt to do things a bit differently by using browsers already installed on the system where possible.

The goal is to keep things simple. Make a web app, point Umea at it, and run it like a desktop app.