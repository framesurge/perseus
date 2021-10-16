# Security Considerations of Plugins

Perseus' plugins system makes it phenomenally versatile, and allows you to reshape default behavior in ways that are possible in very few other frameworks (especially frameworks built in compiled languages like Rust). However, this comes with a major security risk to your system, because plugins have the power to execute arbitrary code.

## The Risks

If you enable a plugin in your app, it will have the opportunity to run arbitrary code. The actions that plugins take are just functions that they provide, so a plugin could easily be saying that it's adding an extra [static alias](:static-content) while simultaneously installing malware on your computer.

## Precautions

1. **Only ever use plugins that you trust!** Anyone can create a Perseus plugin, and some people may create plugins designed to install malware on your system. Optimally, you should review the code of every plugin that you install.
2. **Never run Perseus as root!** If you run Perseus and any plugins as the root user, a plugin can do literally anything on your computer, which could include installing privileged malware (by which point your computer would be owned by an attacker).

**TL;DR:** don't use shady code, and don't run things with unnecessary privileges in general.
