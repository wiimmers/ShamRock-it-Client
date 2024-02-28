#        ShamRock.it
##       v2.1.1 - *Release binaries only*   

##       Functionality:
ShamRock.it is a simple application created with Rust, HTML, CSS, JavaScript, and the Tauri framework. It functions as 
an IT and Maintenance ticketing suite, while also allowing users to backup user files for transfer to another system. 
ShamRock.it has an additional binary for administrators that creates an encrypted 'assets' file for secure storage and 
use of credentials. ShamRock-it-Secure-Assets is needed to create this file. The 'assets' file with the encrypted information
is placed in the assets direcory under src-tauri and decrypted within ShamRock.it.

###     Ticketing
- Click the submit ticket button
- Fill out form fields
- Click send ticket
- Maintenance tickets are created and emailed from maintenance email
- IT tickets are created with Ninja One API


###      Backup
(Optional) Export bookmarks and passwords and save them to one of the folders being backed up
These folders are: Desktop, Downloads, Documents, Favorites, Videos, Pictures
- Move from ticket page to backup page using Backup User Files button in top right
- Click 'Start Backup'
- Await program finish and navigate to newly created tmp folder to find [user].zip file

---
####    <p style="text-align:center">Authored by: Nick Wimmers</p>

