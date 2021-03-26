import { lazy, Suspense } from 'react'
import logo from './logo.svg';
import './App.css';


let Lazy = lazy(() => import('./Lazy'))

function App() {
  return (
    <Suspense fallback={null}>
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <p>
            Edit <code>src/App.js</code> and save to reload.
          </p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
        </header>

        <Lazy />
      </div>
    </Suspense>
  );
}

export default App;
