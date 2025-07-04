# Tosca

## Introduction 

- Scriptless, AI based
- No code, automation tool for end-to-end test automation
- Integrates with CI/CD tools
- Allows to design functional, automated, end-to-end software tests
- provides real-time risk assessment with business focussed results
- can simulate the behavior of dependent systems if they are unavailable during test execution.

## Features

- Risk based testing
- Load testing
- Test management
- Test data management
- Manual testing
- API testing
- Exploratory testing
- BI & Big data
- Continuous integration
- Service virtualization
- Impact analysis


## Components

### Tosca Commander

- Main component of Tosca
- Enables easy creation, management, execution and analysis of test cases

### Tosca Executor

- Executes the test cases created in Tosca Commander

### Tosca XScan

- Scans the app and saves the technical info of all the elements in modules which is used to identify and steer the screen items.

### Test Repository

- Manages the test data needed for test execution

## Tosca VS Selenium

- Tosca is a commercial tool, while Selenium is an open-source tool.
- Tosca is scriptless, while Selenium requires coding skills.
- Supports all types of app, while Selenium is mainly for web apps.
- Inbuilt reporting and object repository
- Model based test automation in tosca, while selenium is code based.
- Supports API and load testing, while Selenium is mainly for functional testing.

## Scanning

- The elements that needs to be interacted with are called controls.
- Scanning grabs all information on the controls and saves it in modules.

## Modules

- Building blocks of the tests.
- Contains all technical information of the controls.
- We can create modules by scanning the application or by manually creating them.


## Test Cases

- Sequence of actions to be performed on the application.
- Called as Test Steps in Tosca.

### Creating Test cases

- Specify which modules should be used in the test case.
- Drag and drop the modules into the test case.
- Fill in the parameters for the modules. i.e. the values that should be used during the test execution.

### Creating automated test cases

- Identify requirements that you expect from the test.
- Optionally, design logical test structure to set which tests you need to cover the requirements.
- Scan the system under test to create modules.
- Create test cases out of the modules.
- Run tests.

## Workspaces

- A workspace is a collection of test cases, modules, and other artifacts.
- Two types :
    - Single user : Only one person needs access to data.
    - Multi user : Multiple people need access to data.