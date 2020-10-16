import React from 'react';
import './hero.css';
import logo from '../../logo.svg';

class Hero extends React.Component {
  render() {
    return (
      <div className="Hero">
        <img src={logo} className="Hero-logo" alt="logo" />
      </div>
    );
  }
}

export default Hero;
