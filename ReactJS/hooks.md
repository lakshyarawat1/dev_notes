# React Hooks

## Built-in DOM Hooks

### State Hooks

- `useState` :
    - Adds a state variable to a functional component.
    - Gives a setState function to update the state.
    - Parameters:
        - Initial state value.
    - Returns : 
        - Current state value.
        - Function to update the state. 
    - Caveats : 
        - useState can be called only on the top level of the component, not inside loops or conditions.
        - In strict mode, react calls the initializer function twice, only in development mode.
    - setState : 
        - Takes the next state value as parameter.
        - If you pass a function in nextState, it will be treated as an updater function, must be pure.
        - Returns nothing.
        - Caveats : 
            - It updates the state at next render, not immediately.
            - If the next state is the same as the current state, react skips rendering.
            - React batches state updates, i.e. updates the screen after the event handler is finished, prevents unnecessary renders.
            - Doesnot trigger effects.
            - 
    - Important : 
        - If you want to update state based on previous state, use the functional form of setState.
            - For example, `setCount(prevCount => prevCount + 1)`.
            - Doing `setCount(count + 1)` may lead to stale state if multiple updates are queued. As setState gets a pending state, not the latest state.
        - Instead of mutating the object like `form.firstName = 'John'`, replace it with a new Object like `setForm({...form, firstName: 'John'})` to ensure React detects the change.
        - You can reset a state using the `key` prop on a component when rendering lists. Changing the key will remount the component, resetting its state.

- `useReducer` : 
    - Lets us add a reducer to the component.
    - Parameters : 
        - `reducer` : Reducer function that specifies how the state changes based on actions.
            - Must be pure function, takes current state and action as parameters, returns new state.
        - `initialArg` : Initial state value.
        - `init` (optional) : Function to create the initial state from initialArg.
    - Returns :
        - Array with two elements:
        - Current state. InitialArg for initial render.
        - Dispatch function to send actions to the reducer.
    - Caveats :
        - Same as useState
    - `dispatch` function : 
        - Returned by useReducer hook.
        - Lets you update the state and trigger a re-render.
        - Parameters : 
            - `action` : An object describing the change to be made. Must have a `type` property.
        - Returns nothing.
        - Caveats : 
            - Updates state at next render.
            - Skips rendering if new state is same as current state.
            - Batches state updates.
    - `reducer` function : 
        - Convention to use `switch case` statements to define different logic for different actions.
        - Each action describes single interactive, even if it leads of multiple changes.
        
- `useState` vs `useReducer` : 
    - useState writes less code
    - useState is easy to read and debug
    - useReducer is pure function, i.e. it cannot be tested in isolation
    
### Context Hooks

- Receives information from distant parents without passing it as prop.
- `useContext` : 
    - Lets you read and subscribe to a context of your component.
    - Parameters : 
        - The context previously created using `createContext` function.
    - Returns : 
        - Returns the context value for the calling component.
    - Caveats : 
        - Contexts are not affected by the providers returned from the same component.
        - Automatically re-renders all the components that use a particular context starting from the provider that recieves a different value. Skips if the value is same.
        - 