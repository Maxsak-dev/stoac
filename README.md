# stoac - Store a command
Stoac is a quick cli helper created for a very simple purpose: Keeping your shell commands organized. 

You can think of stoac as a vip history database with tagging abilites. 
It uses a Key-Value based database that will simply store any command you wish with a given tag. 
You can either store commands manually (as text) or store commands directly from you bash or zsh history using the index (similar to using e.g. "!12"). 

After you build your database you can simply execute your templates with the load flag and a given tag. 

## Example Usage

Let's say you want to create a new bootable USB stick to try out the next cool distro and you remember how easy it was last time you did that using the command line!
However you don't remember the tool you were using for it. 
Of course you could skim through your history file to try to find it but this can sometimes be very hard as some commands are not really recognizable like the dd command for example:
~~~bash
dd bs=4M if=/home/user/Downloads/linux.iso of=/dev/disk/by-id/myusb conv=fsync oflag=direct status=progress
~~~

Stoac to the rescue!
Just store the command prototype to your database with the tag "iso" and next time you can just type
~~~bash
stoac -l iso
~~~
fill the needed paths and execute the command! Done!

A general overview of the usage can be found in the following video:

![Usage Gif](demo/demo.gif)

## Alternatives

This program serves a very general purpose which can also be achieved with a multitude of other tools. 

A simple solution is to enter a command into you shell and using its code comment feature "#" as a tag. 
Then you can simply search the history for these tags. 
[This stackexchange discussion](https://unix.stackexchange.com/questions/26245/how-to-quickly-store-and-access-often-used-commands) is a very good discussion about the subject of storing and accessing commands, have a look there!

## Outlook

This project is very open to contributions in any forms (code, ideas, critizism). 
It currently uses a Rust implementation but the idea behind the project would also fit other implementation types very well especially direct implementation into the shell instead of a seperate program. 
The dream would be to have tagging abilities in the history itself and then for example be able to type some shorthand (like !12) but with tags instead of history indexes. 
Maybe a zshell plugin could be the solution for this in the future. 
For now this is written as a very rudamentary helper for my own needs, so maybe it also fits yours. :)
