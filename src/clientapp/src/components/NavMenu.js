import React from 'react';
import {Navbar, Button, NavbarBrand, NavbarToggler, NavItem, NavLink, UncontrolledDropdown, DropdownToggle, DropdownMenu, DropdownItem, NavbarText, Collapse, Nav } from 'reactstrap';

class NavMenu extends React.Component {
    constructor(props){
        super(props);
        this.state = {
            isOpen: false,
        }

        this.syncCallback = this.props.syncCallback;
        this.toggleRunCallback = this.props.toggleRunCallback;
    }

    toggle(){
        //this.setState({isOpen: this.state.isOpen ? false : true});
    }

    render() {
        if(this.props.Running == true){
            var running1 = "Running";
            var running2 = "running";
        }else{
            var running1 = "Stopped";
            var running2 = "stopped";
        }

        if(this.props.isModified == true){
            var button = <Button onClick={this.syncCallback} color="success">Save</Button>
        }else{
            var button = <Button disabled color="secondary">Save</Button>
        }

        return (
            <Navbar className="planner-navbar" color="dark" dark expand="md">
                <NavbarBrand><img className="logo" src="logo.png"/>AlexBox</NavbarBrand>
                <Nav className="ml-auto" navbar>
                    <NavItem>
                        {button}&ensp;
                        <Button onClick={this.toggleRunCallback} color="secondary" disabled>Doesn't Work {/*running1*/ /*<span className={"circle "+running2}></span>*/}</Button>
                    </NavItem>
                </Nav>
            </Navbar>
        );
    }
  }

  export default NavMenu;