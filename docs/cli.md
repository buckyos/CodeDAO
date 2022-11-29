

# CodeDAO cli schedule


```
codedao create <name>
codedao push
codedao pull
codedao view 



# read
codedao cat <oid>
````

## create repostiory

If in a dir with .git, cli may auto complete the name.

`code-dao-cli init <repository name>`

This cli can create a repository for path: peopleID/<name>


`code-dao-cli init -h`


## FIX
run in repository sub dir, match target `.git`



## development
`codedao init` auto init cwd dir name 's repository,  and get a line to ask use(Y/N) 
`codedao init <name>`


## upload blob file
create task call in runtime (trigger the task start in ood)


# codedao transformer
read rootstate -> and generate the .git(as a cached) in OOD
use git2 to open(read) .git 
drop the git base cmd code

struct transform
  repo_name
  path
fn read
  commit
   .git/objects/xx/xxxx
  tree
  <>
  blob, get blob file


# pref testcase 
  
