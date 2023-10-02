// import React from 'react';
import './App.css';
import Header from './Header';
import MessageForm from './MessageForm';
import TransactionList from './TransactionList';

function App() {
  return (
    <div className="App">
      <Header />
      <main>
        <MessageForm />
        <TransactionList />
      </main>
    </div>
  );
}

export default App;
