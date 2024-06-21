import React from 'react';
import './App.css';
import {BrowserRouter as Router, Routes, Route} from "react-router-dom"
import Layout from './components/Login/Layout';
import Login from './page/Login';

function App() {
  return (
    <Router>
      <div className="App">
        <Routes>
          <Route 
            path='/login' 
            element={
              <Layout>
                <Login/>
              </Layout>
          }>

          </Route>
          <Route path='/register' element={NaN}></Route>
        </Routes>
     </div>
    </Router>
  );
}

export default App;
