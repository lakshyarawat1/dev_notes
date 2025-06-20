# React JS Setup

## Steps 

### Create React App

- CRA is a cli tool to set up a new React project with a good default configuration.
- Install it globally using npm:
```bash
  npm install -g create-react-app
```
- Create a new React app:
```bash
  npx create-react-app my-app
```
- Cli tool can be changed according to the framework you use like, create-next-app, create-vite, etc.


## Build from Scratch

### Install a build tool

- provides features to package and run source code
- provides dev server for local development and build command.

- Vite : 
    - provides faster and leaner approach
    - install using npm:
    ```bash
      npm create vite@latest my-app --template react
    ```
    - Lot of plugins
    - Supports hot module replacement (HMR) out of the box.

- Parcel : 
    - Scalable architecture
    - install using npm:
    ```bash
      npm install --save-dev parcel
    ```
    - supports fast refresh and styling

more...

### Build common app patterns

- Routing : Use React router / Tanstack router
- Data fetching : React Query / Tanstack Query or graphql
- Code splitting : Use dynamic imports

## Typescript

- Add definition types to js codebases.
- Install typescript:
```bash
  npm install --save-dev typescript @types/react @types/react-dom
```
## React Compiler

- React compiler is used to convert JSX code into JavaScript.
- Installation script : 
```bash
  npm install -D babel-plugin-react-compiler@rc eslint-plugin-react-hooks@^6.0.0-rc.1
```
- React compiler automatically memoizes the code i.e. it does not need recomputing if their input has not changed.