# Study Blocker 🙇‍♂️🚫

Built by @blind_nabler

An application designed to block distracting websites to allow you to do your best work.

I built this as a side project because I love working with rust. But I am also building this for my friends that use linux and want a simple GUI tool to manage what sites are blocked and for how long. I used to use SelfControl but it is only available for MacOS, but since I built this with rust and egui this is no longer an issue.

# How to use

To use this application, you must launch it as root as we are modifying the /etc/hosts file. This is the easiest way to work around the permissions here, maybe there is some other method I am missing such as the `$HOSTALIASES` env variable we could set, however this appears to not work on everything and doesn't work on macOS from my personal experiences.

After you launch it, simply create a new line with the address of each site you want to block and for how long.
