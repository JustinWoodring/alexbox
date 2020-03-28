import React from 'react';
import {BrowserRouter as Router, Link, Route, Switch} from 'react-router-dom';
import logo from './logo.svg';
import Base from './components/Base.js';
import About from './components/About.js';
import './App.css';

function App() {
  return (
    <Router>
		<Switch>
			<Route path="/" component={Base} />
			<Route path="/about" component={About} />
		</Switch>
    </Router>
  );
}

export default App;
