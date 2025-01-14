## Command Line Tool

- Install using `npm install -g typescript`
- Check version using `tsc -v`

### Options

- `tsc <filename>.ts` - Compiles the file
- `tsc -w <filename>.ts` - Watches for changes and compiles
- `tsc -init` - Creates a `tsconfig.json` file / Initializes a project
- `tsc --all` - Shows all compiler Options
- `tsc --listFilesOnly` - Prints the name of files that are part of the Compilation
- `tsc --project` - Compiles the project in the directory
- `tsc --showConfig` - Shows the configuration of the project
- `tsc --build` - Builds one or more projects and their dependencies
- `tsc --clean` - Deletes the outputs of the project
- `tsc --dry` - Shows what would be built / deleted
- `tsc --force` - Forces the compiler to build the project

Watch Options

- `tsc --excludeDirectories` - Directories to exclude
- `tsc --excludeFiles` - Files to exclude
- `tsc --fallbackPolling` - Specify what approach the watcher should use if the system runs out of native file watchers 
- `tsc --synchronousWatchDirectory` - Synchronously call callbacks and update the state of directory watchers 
- `tsc --watch` - Watch input files

Compiler Flags

- `tsc --allowArbitraryExtensions` - Enable importing files with any extensions
- `tsc --allowImportingTsExtensions` - Allow importing `.ts` files
- `tsc --allowJS` - Allow JavaScript files to be compiled
