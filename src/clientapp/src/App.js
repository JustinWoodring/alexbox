import React from 'react';
import {BrowserRouter as Router, Link, Route, Switch} from 'react-router-dom';
import Base from './components/Base.js';
import './App.css';

function App() {
  return (
    <Router>
		<Switch>
			<Route path="/" component={Base} />
		</Switch>
    </Router>
  );
}

export default App;
