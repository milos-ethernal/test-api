// Header.js
import React, { useState, useEffect } from 'react';

function Header() {
  const [headerContent, setHeaderContent] = useState("");

  useEffect(() => {
    fetch('/api/header')
      .then((response) => response.json())
      .then((data) => setHeaderContent(data))
      .catch((error) => console.error('Error fetching header content:', error));
  }, []);

  return (
    <header className="App-header">
      <h1>{headerContent}</h1>
    </header>
  );
}

export default Header;
