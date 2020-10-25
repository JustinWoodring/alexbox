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
          serviceRunning: true,
          modified: false,
          config: {ui_name: "Alexbox", ui_logo: "logo.png", ui_colors: {}},
        }
        this.syncCallback = this.syncCallback.bind(this);
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
      var self = this;
      setInterval(async function(){axios.get('/service/scheduler')
          .then(res => {
              if (res.data.running == true){
                self.setState({serviceRunning: true});
              }else{
                self.setState({serviceRunning: false});
              }
      }).catch((error) => {
        self.setState({serviceRunning: false});
      })}, 5000);

      axios.get('/config').then(res => {
        this.setState({config: res.data});
        document.getElementById("title").innerText=res.data.ui_name;
        var i = 1;
        Object.values(res.data.ui_colors).map((element, index) => {
          if(i <= 20){
            let styleSheet = document.styleSheets[0];
            styleSheet.insertRule(".tile.tile-color-"+i+" { background-color: "+element[1]+"}");
          }
          i++;
        })
      }).catch((error) => {
      })
    }

    render() {
      return (
        <div>
          <NavMenu 
            config={this.state.config} 
            syncCallback={this.syncCallback} 
            isModified={this.state.modified} 
            isRunning={this.state.serviceRunning} 
            toggleRunCallback={this.toggleRunCallback}
          />
          <Editor 
            config={this.state.config}
            syncCallback={this.syncCallback} 
            toggleModifiedCallbackTrue={this.toggleModifiedCallbackTrue} 
            toggleModifiedCallbackFalse={this.toggleModifiedCallbackFalse} 
            shouldSync={this.state.shouldSync}
          />
          <Footer/>
        </div>
      );
    }
  }

  export default Base;