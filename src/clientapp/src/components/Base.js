import React from 'react';
import NavMenu from './NavMenu.js';
import Editor from './Editor.js';
import Footer from './Footer.js';  
import axios from 'axios';
import '../style/base.css';

class Base extends React.Component {
    constructor(props){
        super(props);
        this.state = {
          isLoggedIn: true,
          shouldSync: false,
          serverRunning: false,
          modified: false,
          config: null,
        }
        this.syncCallback = this.syncCallback.bind(this);
        this.toggleRunCallback = this.toggleRunCallback.bind(this);
        this.toggleModifiedCallbackTrue = this.toggleModifiedCallbackTrue.bind(this);
        this.toggleModifiedCallbackFalse = this.toggleModifiedCallbackFalse.bind(this);
    }
    
    syncCallback(){
      var shouldSync = this.state.shouldSync ? false : true;
      this.setState({shouldSync: shouldSync});
    }

    toggleModifiedCallbackTrue(){
      this.setState({modified:true})
    }
    toggleModifiedCallbackFalse(){
      this.setState({modified:false})
    }

    componentDidMount(){
      //Attempt fetch of contents.
      /*setInterval(axios.get('/running')
          .then(res => {
              if (res.data == true){
                this.setState({serverRunning: true});
              }else{
                this.setState({serverRunning: false});
              }
      }).catch((error) => {
        this.setState({serverRunning: false});
      }), 2000);*/
      /*'Content-Type': 'application/json', 'Cache-Control': 'no-cache'*/

      axios.get('/config').then(res => {
        this.setState({config: res.data});
        document.getElementById("title").innerText=res.data.ui_name;
      }).catch((error) => {
          this.setState({serverRunning: false});
      })
    }


    toggleRunCallback(){
      this.setState({serverRunning: true});
      /*axios.post('/running')
          .then(res => {
              if (res.data == true){
                this.setState({serverRunning: false});
              }else{
                this.setState({serverRunning: true});
              }
            }
          ).catch((error) => {
            this.setState({serverRunning: false});
          })*/
    }

    render() {
      return (<div><NavMenu config={this.state.config} syncCallback={this.syncCallback} isModified={this.state.modified} Running={this.state.serverRunning} toggleRunCallback={this.toggleRunCallback}/><Editor syncCallback={this.syncCallback} toggleModifiedCallbackTrue={this.toggleModifiedCallbackTrue} toggleModifiedCallbackFalse={this.toggleModifiedCallbackFalse} shouldSync={this.state.shouldSync}/><Footer/></div>);
    }
  }

  export default Base;