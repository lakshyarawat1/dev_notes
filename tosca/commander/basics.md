# Tosca Commander

## Overview

- User interface of Tricentis Tosca

## Workspaces

- A workspace is a collection of test cases, modules, and other artifacts.
- Two types :
    - Single user : Only one person needs access to data.
    - Multi user : Multiple people need access to data.

### Anatomy of a Workspace

- Ribbon Menu : Has static and dynamic menus
- Tabs : Object types are grouped into tabs, e.g. Test Cases, Modules, Requirements, etc.
- Project Tree : Hierarchical view of all objects in the workspace.
- Details : Detailed information of the selected object in the Project Tree.
- Properties : Properties of the selected object in the Project Tree.
- Status Bar : Progress of the action that Tosca Commander currently performs

### Workspace Customization

- You can customize the workspace by adding or removing tabs, changing the layout, and modifying the properties displayed in the Details and Properties sections.

### Command Line

- Tosca Commander can be launched from the command line with various options.
- A workspace can be opened from the command line using the `-workspace` option followed by the path to the workspace file.
- Change language of Tosca Commander using the `-language` option followed by the language code.
- Login to Tosca Commander from the command line using the `-login` option followed by the username and password.
- Jump to a specific object in the workspace using the `-jumpTo` option followed by the object ID.
- Execution mode can be set from the command line using the `-executionMode` option followed by the mode (e.g., `-executionMode=TestCase`).

## First Tests

- Create a new single-user workspace.
- Add workspace template if needed.
- 