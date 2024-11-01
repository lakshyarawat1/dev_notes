# Angular

## Introduction

- Collection of frontend developer tool.
- Design framework and development platform
- Component based architecture
- Introduced in 2010
- Angular 2 : 2016 Component based.
- Angular 7 : 2018 Angular Material
- Angular 15 : 2022 Stand alone components.

### Features

- Cross-platform applications
- Dependency injection and declarative templates.
- Built in support for HTTP, AJAX, and observables.
- Two-way binding

### Anatomy

- Module : Special class with @ng module decorator.
- Component : has real HTML,CSS,JS code.
- Directive : Special class, add additional behavior to app.
- Service
- Router 

### Architecture

- Module > Templates
            Components
            Directives

- App module is the base.
- Component is a directive with a template.
- Components are created using class structure.

## Modules

- Containers
- Root module bootstraps the app, (usually AppModule)
- Feature modules organize features.(ex : UserModule, ProductModule)
- Modules declare components, directives and pipes, making them usable within the module
- Managing dependency injection services.

## Components 

- Building blocks
- Displaying the UI and handling interactions.
- Components : 
    - Template (HTML) 
    - Class (Typescript) : Logic and data for the components
    - Styles(CSS) 
- Components communicate with each other using inputs and outputs and are organized heirarchically, 

## Templates and Directives

- Template define HTML structure of the view.
- Binding data and reacting to user events

### Data binding

- Interpolation ({{  }}) : Binds component data with the template
- Property Binding ([property]='value') : with DOM Property
- Event Binding ((event) ='method()') : Listens for events from the DOM.
- Two-way Binding ([(ngModel)]='property') : Keeps component property and for element in sync.

### Directives

- Structural Directives ( eg : *ngIf, *ngFor ) : Change the DOM layout
- Attribute Directives ( eg : [ngClass] ) : change appearence and behavior of the element.

## Angular Services

- Services are classes that contain common code and data that can be shared across the app
- Enhance reusability and modularity
- Tasks like fetching data from API, handling user authentication, managing state or other logic.
- Use cases : 
    - Separation of concerns : seperate business logic from view layer (components),keeping components lean.
    - reusability 
    - Dependency injection (DJ) : Services are often injected into components and other services enabling centralized control of how and when services are instantized and used.
    - Testing easier
    - HTTP Requests
    - State Management
    - Utility Functions

### Creating services

- Services are simple Typescript classes
- Decorated with @Injectable() to make them usable with Dependency Injection System
- Syntax : 
    - ```
    @Injectable({
        providedIn : 'root' // app-wide usage
        })
    ```

## Routing and Navigation

- Angular Router manages routing 
- Maps URL to components, navigate without reloading the page.
- Routes are defined in routing module with route guards to control access to routes.


## pipes

- Transform data in the template
- Built in pipelines are like DatePipe, UpperCasePipe
- CustomPipes 

## Forms

### Template-driven forms 

### Reactive Forms

## HttpClient Module

- Provides simplified API to interact with backend APIs, manage HTTP request and handle responses
- Observables from RxJS are used to handle async operations, making it easy to handle streams of data.

## Angular CLI

- Creating, Building, and managing Angular apps.

### Commands

- add (collectionName) : Adds external library to project.
    - defaults : Disable interactive input prompts taking defaults. Default:False
    - dry-run : Run through and reports activity without writing out results.
    - force : Overwriting existing files
    - interactive
    - registry : npm registry to use

- analytics : 
    - Statistics and analytics
    - disable
    - enable

- build : For building the project. Uses esbuild. 
    - Configs are in angular.json
    

## App State Management

- Uses NgRx and RxJS for managing complex states in larger apps.