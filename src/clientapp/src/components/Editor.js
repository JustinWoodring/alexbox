import React, {useState} from 'react';
import {Table, Form, Input, Label, InputGroup, FormGroup, Button, UncontrolledTooltip, Spinner} from 'reactstrap';
import { TabContent, TabPane, Nav, NavItem, NavLink, InputGroupAddon, InputGroupText } from 'reactstrap';
import classnames from 'classnames';
import axios from 'axios';


class Editor extends React.Component {
    constructor(props){
        super(props);
        this.state = {
            gridItems: [],
            selectedTile: null,
            isSyncing: true,
            linestyle: {
                top: (50+1)+"px",
            },
            dragging:false,
            draggingTimer: null,
            item: {day: null, start_time: null}
        }


        this.handleGridMouseDown = this.handleGridMouseDown.bind(this);
        this.squareClick = this.squareClick.bind(this);
        this.handleTitle = this.handleTitle.bind(this);
        this.handleCommand = this.handleCommand.bind(this);
        this.handleStartTime = this.handleStartTime.bind(this);
        this.handleStartTimePositive = this.handleStartTimePositive.bind(this);
        this.handleStartTimeNegative = this.handleStartTimeNegative.bind(this);
        this.handleDuration = this.handleDuration.bind(this);
        this.handleDurationPositive = this.handleDurationPositive.bind(this);
        this.handleDurationNegative = this.handleDurationNegative.bind(this);
        this.handleColor = this.handleColor.bind(this);
        this.handleDelete = this.handleDelete.bind(this);
        this.handlePlayNow = this.handlePlayNow.bind(this);
        this.toggleModifiedCallbackTrue = this.props.toggleModifiedCallbackTrue;
        this.toggleModifiedCallbackFalse = this.props.toggleModifiedCallbackFalse;
        this.update = this.update.bind(this);
        this.beginSquareDrag = this.beginSquareDrag.bind(this);
        this.handleGridMouseUp = this.handleGridMouseUp.bind(this);
        this.handleGridMouseMove = this.handleGridMouseMove.bind(this);
    }

    componentDidMount(){
        var self = this;
        window.addEventListener("mouseup", function(){self.setState({dragging:false})});
        //Attempt fetch of contents.
        const interval = setInterval(axios({
                method: 'get',
                url: '/tile',
                config: { headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-cache' } } 
            })
            .then(res => {
                var gridItems = res.data;
                var normalizedGridItems = [];
                for (var i =0; i<gridItems.length; i++){
                    var item = gridItems[i];
                    item.modified = false;
                    item.deleted = false;
                    normalizedGridItems.push(item);
                }
                this.setState({gridItems: normalizedGridItems, isSyncing: false});
                clearInterval(interval);
        }), 2000);
        const interval2 = setInterval(this.update, 200);
    }

    update(){
        var date = new Date()
        var number = 50 + (50*(date.getHours()+(date.getMinutes()/60)));
        this.setState({linestyle: {top: number+"px",}});
        var modified = false;
        for (var i =0; i<this.state.gridItems.length; i++){
            if(this.state.gridItems[i].modified==true){
                modified=true;
            }
        }
        if(modified==true){
            this.toggleModifiedCallbackTrue();
        }else{
            this.toggleModifiedCallbackFalse();
        }
    }

    handleGridMouseDown(e){
        e.preventDefault();
        e.stopPropagation();
        var rect = e.target.getBoundingClientRect();
        var x = e.clientX - rect.left;
        var y = e.clientY - rect.top;

        if((x/((rect.right-rect.left)/8)) >= 1 && (y/((rect.bottom-rect.top)/50)) >= 2){
            if (!this.state.dragging){
                var item = {};
                item.id = -1
                item.title = "No Title"
                item.command = ""
                item.modified = true;
                item.day = Math.floor(x/((rect.right-rect.left)/8))-1
                item.start_time = Math.floor(y/((rect.bottom-rect.top)/50))/2-1
                item.duration = .5
                item.deleted = false

                var colors = ['1']
                var i  = Math.floor(Math.random() * 6);
                item.color = colors[0];
                var gridItems = this.state.gridItems;
                gridItems.push(item)
                this.setState({gridItems: gridItems, selectedTile: gridItems.length-1});
            }
        }
    }
    
    handleGridMouseUp(e){
        e.preventDefault();
        e.stopPropagation();
        var rect = e.target.getBoundingClientRect();
        var x = e.clientX - rect.left;
        var y = e.clientY - rect.top;

        if((x/((rect.right-rect.left)/8)) >= 1 && (y/((rect.bottom-rect.top)/50)) >= 2 && this.state.dragging){
            var oldGridItems = this.state.gridItems;
            var item = oldGridItems[this.state.selectedTile];
            var selectedTile = this.state.selectedTile;
            var duration = item.duration
            var new_day = Math.floor(x/((rect.right-rect.left)/8))-1
            var new_start_time = Math.floor(y/((rect.bottom-rect.top)/50))/2-1

            if(new_start_time % .5 == 0 && new_start_time >= 0 && (duration + new_start_time) <= 24){
                var ok = true;
                for(var i = 0;i<oldGridItems.length;i++){
                    if(i!=selectedTile){
                        var tile = item;
                        var other_tile = oldGridItems[i];
                        if(other_tile.deleted != true){
                            if(
                                (
                                    (
                                        (new_start_time < other_tile.start_time) && 
                                        (new_start_time + duration > other_tile.start_time)
                                    ) ||
                                    (
                                        new_start_time == other_tile.start_time
                                    ) ||
                                    (
                                        (other_tile.start_time < new_start_time) && 
                                        (other_tile.start_time+other_tile.duration > new_start_time)
                                    )
                                ) && other_tile.day == new_day
                            ){
                                ok = false;
                            }
                        }
                    }
                }
                if(ok){
                    oldGridItems[selectedTile].start_time = new_start_time;
                    oldGridItems[selectedTile].day = new_day;
                    oldGridItems[selectedTile].modified = true;
                    this.setState({gridItems: oldGridItems});
                }
            }
        }
        clearTimeout(this.state.draggingTimer);
        this.setState({dragging:false});
    }

    handleGridMouseMove(e){
        e.preventDefault();
        e.stopPropagation();
        var rect = e.target.getBoundingClientRect();
        var x = e.clientX - rect.left;
        var y = e.clientY - rect.top;

        if((x/((rect.right-rect.left)/8)) >= 1 && (y/((rect.bottom-rect.top)/50)) >= 2 && this.state.dragging){
            var oldGridItems = this.state.gridItems;
            var item = oldGridItems[this.state.selectedTile];
            var selectedTile = this.state.selectedTile;
            var duration = item.duration
            var new_day = Math.floor(x/((rect.right-rect.left)/8))-1
            var new_start_time = Math.floor(y/((rect.bottom-rect.top)/50))/2-1

            if(new_start_time % .5 == 0 && new_start_time >= 0 && (duration + new_start_time) <= 24){
                var ok = true;
                for(var i = 0;i<oldGridItems.length;i++){
                    if(i!=selectedTile){
                        var tile = item;
                        var other_tile = oldGridItems[i];
                        if(other_tile.deleted != true){
                            if(
                                (
                                    (
                                        (new_start_time < other_tile.start_time) && 
                                        (new_start_time + duration > other_tile.start_time)
                                    ) ||
                                    (
                                        new_start_time == other_tile.start_time
                                    ) ||
                                    (
                                        (other_tile.start_time < new_start_time) && 
                                        (other_tile.start_time+other_tile.duration > new_start_time)
                                    )
                                ) && other_tile.day == new_day
                            ){
                                ok = false;
                            }
                        }
                    }
                }
                if(ok){
                    oldGridItems[selectedTile].start_time = new_start_time;
                    oldGridItems[selectedTile].day = new_day;
                    oldGridItems[selectedTile].modified = true;
                    this.setState({gridItems: oldGridItems});
                }
            }
        }
    }

    squareClick(e,index){
        e.preventDefault();
        e.stopPropagation();
        if (!this.state.dragging) {
            if(this.state.selectedTile == index){
                this.setState({selectedTile:null});
            }else{
                this.setState({selectedTile:index});
            }
        }
        clearTimeout(this.state.draggingTimer);
        this.setState({dragging:false});
    }

    beginSquareDrag(e,index){
        e.preventDefault();
        e.stopPropagation();
        var self =this
        var day = this.state.gridItems[index].day;
        var start_time = this.state.gridItems[index].start_time;
        this.setState({draggingTimer: setTimeout(function(){self.setState({dragging:true, selectedTile:index})}, 125), item: {day: day, start_time: start_time}});
        if(this.state.selectedTile != index){
            clearTimeout(this.state.draggingTimer);
            this.setState({dragging: false, item: {day: day, start_time: start_time}});
            
        }
    }

    sync(){
        this.setState({isSyncing: true});
        var elements_to_delete = [];
        var elements_to_update = [];
        var elements_to_create = [];
        var gridItems = this.state.gridItems;
        for(var i = 0;i<gridItems.length;i++){
            if(gridItems[i].modified==true){
                if(gridItems[i].id != -1 && gridItems[i].deleted == true){
                    var object = new Object();
                    object.id = gridItems[i].id.toString();
                    elements_to_delete.push(object);
                }else if(gridItems[i].id != -1 && gridItems[i].deleted != true){
                    var object = new Object();
                    object.id = gridItems[i].id.toString();
                    object.title = gridItems[i].title.toString();
                    object.command = gridItems[i].command.toString();
                    object.day = gridItems[i].day.toString();
                    if(gridItems[i].start_time*2 % 2==1){
                        if(gridItems[i].start_time<10){
                            object.start_time = "1970-01-01 0"+(gridItems[i].start_time-.5)+":30:00";
                        }else{
                            object.start_time = "1970-01-01 "+(gridItems[i].start_time-.5)+":30:00";
                        }
                    }else{
                        if(gridItems[i].start_time<10){
                            object.start_time = "1970-01-01 0"+(gridItems[i].start_time)+":00:00";
                        }else{
                            object.start_time = "1970-01-01 "+(gridItems[i].start_time)+":00:00";
                        }
                    }
                    object.duration = gridItems[i].duration.toString();
                    object.color = gridItems[i].color.toString();
                    elements_to_update.push(object);
                }else if(gridItems[i].id == -1){
                    var object = new Object();
                    object.title = gridItems[i].title.toString();
                    object.command = gridItems[i].command.toString();
                    object.day = gridItems[i].day.toString();
                    if(gridItems[i].start_time*2 % 2==1){
                        if(gridItems[i].start_time<10){
                            object.start_time = "1970-01-01 0"+(gridItems[i].start_time-.5)+":30:00";
                        }else{
                            object.start_time = "1970-01-01 "+(gridItems[i].start_time-.5)+":30:00";
                        }
                    }else{
                        if(gridItems[i].start_time<10){
                            object.start_time = "1970-01-01 0"+(gridItems[i].start_time)+":00:00";
                        }else{
                            object.start_time = "1970-01-01 "+(gridItems[i].start_time)+":00:00";
                        }
                    }
                    object.duration = gridItems[i].duration.toString();
                    object.color = gridItems[i].color.toString();
                    elements_to_create.push(object);
                }
            }
        }
        axios({
            method: 'delete',
            url: '/tile',
            data: elements_to_delete,
            config: { headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-cache' } }
        }).then(response => {
            var self1 = this;
            axios({
                method: 'put',
                url: '/tile',
                data: elements_to_update,
                config: { headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-cache' } }
            }).then(response => {
                var self2 = self1;
                axios({
                    method: 'post',
                    url: '/tile',
                    data: elements_to_create,
                    config: { headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-cache'  } }
                }).then(response => {
                    var self3 = self2;
                    const interval = setInterval(axios.get('/tile')
                        .then(res => {
                            var gridItems = res.data;
                            var normalizedGridItems = [];
                            for (var i =0; i<gridItems.length; i++){
                                var item = gridItems[i];
                                item.modified = false;
                                item.deleted = false;
                                normalizedGridItems.push(item);
                            }
                            self3.setState({gridItems: normalizedGridItems, isSyncing: false, selectedTile: null});
                            clearInterval(interval);
                        }), 2000);
                }).catch(function(error) {
                    alert("Creating a thing has failed somehow.")
                    console.log(error);
                });
            }).catch(function(error) {
                alert("Updating a thing has failed somehow.")
                console.log(error);
            });
        }).catch(function(error) {
            alert("Deleting a thing has failed somehow.")
            console.log(error);
        });
    }

    handleTitle(e, index){
        e.preventDefault();
        var gridItems = this.state.gridItems;
        gridItems[index].title = e.target.value;
        gridItems[index].modified = true;
        this.setState({gridItems: gridItems});
    }

    handleCommand(e, index){
        e.preventDefault();
        var gridItems = this.state.gridItems;
        gridItems[index].command = e.target.value;
        gridItems[index].modified = true;
        this.setState({gridItems: gridItems});
    }

    handleDuration(e, index){
        e.preventDefault();
        var gridItems = this.state.gridItems;
        var new_duration = parseFloat(e.target.value);

        if(e.target.value % .5 == 0 && e.target.value >= .5 && (gridItems[index].start_time + new_duration) <= 24){
            var ok = true;
            for(var i = 0;i<gridItems.length;i++){
                if(i!=index){
                    var tile = gridItems[index];
                    var other_tile = gridItems[i];
                    if(other_tile.deleted != true){
                        if(other_tile.start_time > tile.start_time && other_tile.start_time < (tile.start_time + new_duration) && other_tile.day == tile.day){
                            ok = false;
                        }
                    }
                }
            }
            if(ok){
                gridItems[index].duration = new_duration;
                gridItems[index].modified = true;
                this.setState({gridItems: gridItems});
            }
        }
    }

    handleDurationPositive(e, index){
        e.preventDefault();
        var gridItems = this.state.gridItems;
        var new_duration = parseFloat(gridItems[index].duration)+.5;

        if(new_duration % .5 == 0 && new_duration >= .5 && (gridItems[index].start_time + new_duration) <= 24){
            var ok = true;
            for(var i = 0;i<gridItems.length;i++){
                if(i!=index){
                    var tile = gridItems[index];
                    var other_tile = gridItems[i];
                    if(other_tile.deleted != true){
                        if(other_tile.start_time > tile.start_time && other_tile.start_time < (tile.start_time + new_duration) && other_tile.day == tile.day){
                            ok = false;
                        }
                    }
                }
            }
            if(ok){
                gridItems[index].duration = new_duration;
                gridItems[index].modified = true;
                this.setState({gridItems: gridItems});
            }
        }
    }

    handleDurationNegative(e, index){
        e.preventDefault();
        var gridItems = this.state.gridItems;
        var new_duration = parseFloat(gridItems[index].duration)-.5;

        if(new_duration % .5 == 0 && new_duration >= .5 && (gridItems[index].start_time + new_duration) <= 24){
            var ok = true;
            for(var i = 0;i<gridItems.length;i++){
                if(i!=index){
                    var tile = gridItems[index];
                    var other_tile = gridItems[i];
                    if(other_tile.deleted != true){
                        if(other_tile.start_time > tile.start_time && other_tile.start_time < (tile.start_time + new_duration) && other_tile.day == tile.day){
                            ok = false;
                        }
                    }
                }
            }
            if(ok){
                gridItems[index].duration = new_duration;
                gridItems[index].modified = true;
                this.setState({gridItems: gridItems});
            }
        }
    }

    handleStartTime(e, index){
        e.preventDefault();
        var gridItems = this.state.gridItems;
        var new_start_time = parseFloat(e.target.value);

        if(new_start_time % .5 == 0 && new_start_time >= 0 && (gridItems[index].duration + new_start_time) <= 24){
            var ok = true;
            for(var i = 0;i<gridItems.length;i++){
                if(i!=index){
                    var tile = gridItems[index];
                    var other_tile = gridItems[i];
                    if(other_tile.deleted != true){
                        if(
                            (
                                (
                                    (new_start_time < other_tile.start_time) && 
                                    (new_start_time + tile.duration > other_tile.start_time)
                                ) ||
                                (
                                    new_start_time == other_tile.start_time
                                ) ||
                                (
                                    (other_tile.start_time < new_start_time) && 
                                    (other_tile.start_time+other_tile.duration > new_start_time+tile.duration)
                                )
                            ) && other_tile.day == tile.day
                        ){
                            ok = false;
                        }
                    }
                }
            }
            if(ok){
                gridItems[index].start_time = new_start_time;
                gridItems[index].modified = true;
                this.setState({gridItems: gridItems});
            }
        }
    }

    handleStartTimePositive(e, index){
        e.preventDefault();
        var gridItems = this.state.gridItems;
        var new_start_time = parseFloat(gridItems[index].start_time)+.5;

        if(new_start_time % .5 == 0 && new_start_time >= 0 && (gridItems[index].duration + new_start_time) <= 24){
            var ok = true;
            for(var i = 0;i<gridItems.length;i++){
                if(i!=index){
                    var tile = gridItems[index];
                    var other_tile = gridItems[i];
                    if(other_tile.deleted != true){
                        if(
                            (
                                (
                                    (new_start_time < other_tile.start_time) && 
                                    (new_start_time + tile.duration > other_tile.start_time)
                                ) ||
                                (
                                    new_start_time == other_tile.start_time
                                ) ||
                                (
                                    (other_tile.start_time < new_start_time) && 
                                    (other_tile.start_time+other_tile.duration > new_start_time+tile.duration)
                                )
                            ) && other_tile.day == tile.day
                        ){
                            ok = false;
                        }
                    }
                }
            }
            if(ok){
                gridItems[index].start_time = new_start_time;
                gridItems[index].modified = true;
                this.setState({gridItems: gridItems});
            }
        }
    }

    handleStartTimeNegative(e, index){
        e.preventDefault();
        var gridItems = this.state.gridItems;
        var new_start_time = parseFloat(gridItems[index].start_time)-.5;

        if(new_start_time % .5 == 0 && new_start_time >= 0 && (gridItems[index].duration + new_start_time) <= 24){
            var ok = true;
            for(var i = 0;i<gridItems.length;i++){
                if(i!=index){
                    var tile = gridItems[index];
                    var other_tile = gridItems[i];
                    if(other_tile.deleted != true){
                        if(
                            (
                                (
                                    (new_start_time < other_tile.start_time) && 
                                    (new_start_time + tile.duration > other_tile.start_time)
                                ) ||
                                (
                                    new_start_time == other_tile.start_time
                                ) ||
                                (
                                    (other_tile.start_time < new_start_time) && 
                                    (other_tile.start_time+other_tile.duration > new_start_time)
                                )
                            ) && other_tile.day == tile.day
                        ){
                            ok = false;
                        }
                    }
                }
            }
            if(ok){
                gridItems[index].start_time = new_start_time;
                gridItems[index].modified = true;
                this.setState({gridItems: gridItems});
            }
        }
    }

    handleColor(e, index){
        e.preventDefault();
        if(e.target.value > 0 && e.target.value <=  Object.values(this.props.config.ui_colors).length){
            var gridItems = this.state.gridItems;
            gridItems[index].color = e.target.value;
            gridItems[index].modified = true;
            this.setState({gridItems: gridItems});
        }
    }

    handleDelete(e, index){
        e.preventDefault();
        //If no ID simply remove it.
        var oldGridItems = this.state.gridItems;
        if(oldGridItems[index].id==-1){
            var newGridItems = [];
            for(var i = 0;i<oldGridItems.length;i++){
                if(i!=index){
                    newGridItems.push(oldGridItems[i]);
                }
            }
            this.setState({gridItems: newGridItems, selectedTile:null});
        }else{
            oldGridItems[index].deleted=true;
            oldGridItems[index].modified=true;
            this.setState({gridItems: oldGridItems, selectedTile:null});
        }
    }

    handlePlayNow(e, index){
        e.preventDefault();
        //If no ID simply remove it.
        var oldGridItems = this.state.gridItems;
        axios({
            method: 'post',
            url: '/playnow',
            data: {
                command: oldGridItems[index].command,
            },
            config: { headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-cache'  } },
        });
    }

    render() {


        if(this.props.shouldSync){
            this.sync();
            this.props.syncCallback();
        }

        var dayItems = ["#", "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        var rows = [];
        for(var i = 0; i<25;i++){
                rows[i] = [];
                for(var x = 0; x<8;x++){
                    if(i == 0){
                        rows[i].push(<th>{dayItems[x]}</th>);
                    }
                    else if(x == 0){
                        rows[i].push(<th className="side">{i-1}</th>);
                    }
                    else{
                        rows[i].push(<td></td>);
                    }
                }
        }

        return !this.state.isSyncing ? (
            <div className="editor-wrapper">
                <div style={{cursor: (this.state.dragging) ? "move" : "crosshair"}} className="time-table">
                    <div className="time-line" style={this.state.linestyle}></div>
                    <div onMouseDown={this.handleGridMouseDown} onMouseUp={this.handleGridMouseUp} onMouseMove={this.handleGridMouseMove} className="grid">
                        {this.state.gridItems.map((value, index) => {
                            var start = value.start_time*2+3 //We subtracted an extra 1 to make it 0 in start_times.
                            var stop = value.start_time*2+3 + value.duration*2
                            var day = value.day+2;
                            var color = value.color;
                            var title = value.title;
                            if (index == this.state.selectedTile){
                                color = "selected";
                            }
                            if(!value.deleted){
                                return( 
                                <div className={"tile tile-color-"+color} key={index} id={"tile"+index} onMouseMove={(e) => {e.stopPropagation();}} onMouseDown={(e) => this.beginSquareDrag(e, index)} onMouseUp={(e) => this.squareClick(e, index)} style={{gridArea: start + "/" + day + "/" + stop + "/" + (day+1), cursor: (this.state.dragging) ? "no-drop" : "pointer", pointerEvents: (this.state.dragging&&index==this.state.selectedTile) ? "none" : "inherit", opacity: (this.state.dragging&&index==this.state.selectedTile) ? 0.5 : 1}}>
                                    {title}
                                    <UncontrolledTooltip placement="right" target={"tile"+index}>
                                        {title}
                                    </UncontrolledTooltip>
                                </div>)
                            }
                        })}
                    </div>
                    <Table bordered className="table">
                        {rows.map((value, index) => {
                            return <tr>{value}</tr>
                        })}
                    </Table>
                </div>
                <Config 
                    config={this.props.config} 
                    handleDelete={this.handleDelete} 
                    handleTitle={this.handleTitle} 
                    handlePlayNow={this.handlePlayNow} 
                    handleStartTime={this.handleStartTime} 
                    handleStartTimePositive={this.handleStartTimePositive} 
                    handleStartTimeNegative={this.handleStartTimeNegative} 
                    handleDuration={this.handleDuration} 
                    handleDurationPositive={this.handleDurationPositive} 
                    handleDurationNegative={this.handleDurationNegative}
                    handleColor={this.handleColor} 
                    handleCommand={this.handleCommand} 
                    selectedTile={this.state.selectedTile} 
                    gridItems={this.state.gridItems} 
                    className="config-panel"
                />
            </div>
        ) : (<div class="syncSpace"><Spinner style={{margin: "auto", width: '3rem', height: '3rem'}} color="info"/></div>) ;
    }
}

class Config extends React.Component {
    constructor(props){
        super(props);
        this.handleTitle = this.props.handleTitle;
        this.handleCommand = this.props.handleCommand;
        this.handleStartTime = this.props.handleStartTime;
        this.handleStartTimePositive = this.props.handleStartTimePositive;
        this.handleStartTimeNegative = this.props.handleStartTimeNegative;
        this.handleDuration = this.props.handleDuration;
        this.handleDurationPositive = this.props.handleDurationPositive;
        this.handleDurationNegative = this.props.handleDurationNegative;
        this.handleColor = this.props.handleColor;
        this.handlePlayNow = this.props.handlePlayNow;
        this.handleDelete = this.props.handleDelete;
    
        this.state = {
            activeTab:'1',
        }
        this.setActiveTab = this.setActiveTab.bind(this);
    }
    
    onComponentDidMount(){
        this.setActiveTab('1');
    }


    setActiveTab(e){
        if(this.state.activeTab !== e) this.setState({activeTab:e});
    }

    render(){
        if(this.props.selectedTile != null){
            var tile = this.props.gridItems[this.props.selectedTile];
            var time = "";
            
            if(tile.start_time % 1 == .5){
                time+=Math.floor(tile.start_time);
                time+=":30"
            }else{
                time+=Math.floor(tile.start_time);
                time+=":00"
            }

            time+="-";
            
            if((tile.start_time+tile.duration) % 1 == .5){
                time+=Math.floor(tile.start_time+tile.duration);
                time+=":30"
            }else{
                time+=Math.floor(tile.start_time+tile.duration);
                time+=":00"
            }

            time+=" on ";

            switch (tile.day){
                case 0: time+="Sunday";break;
                case 1: time+="Monday";break;
                case 2: time+="Tuesday";break;
                case 3: time+="Wednesday";break;
                case 4: time+="Thursday";break;
                case 5: time+="Friday"; break;
                case 6: time+="Saturday";break;
            }
        }

        var i = 1;
        const colors = []
        Object.values(this.props.config.ui_colors).map((element, index) => {
            if(i <= 20){
                colors.push(<option value={i}>{element[0]}</option>)
                i++;
            }
        })

        return (this.props.selectedTile != null) ? (
            <div className="config-panel">
            <div className="config-panel-inner">
                <div className="form">   
                    <h3>{tile.title}<br/><span style={{fontSize:".8rem"}}> from {time}</span></h3>
                    <Nav tabs>
                        <NavItem>
                            <NavLink className={classnames({ active: this.state.activeTab == '1' })} onClick={() => { this.setActiveTab('1'); }}>
                                Tile
                            </NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink className={classnames({ active: this.state.activeTab == '2' })} onClick={() => { this.setActiveTab('2'); }}>
                                Video
                            </NavLink>
                        </NavItem>
                    </Nav>
                    <br/>
                    <TabContent activeTab={this.state.activeTab}>
                        <TabPane tabId="1">
                            <Form>
                                <FormGroup>
                                    <Label htmlFor="Title">Title</Label>
                                    <Input placeholder="Title" id="Title" maxlength="50"  onChange={(e) => this.handleTitle(e, this.props.selectedTile)} value={this.props.gridItems[this.props.selectedTile].title} type="text"/>
                                </FormGroup>
                                <FormGroup>
                                    <Label htmlFor="StartTime">StartTime (Hrs)</Label>
                                    <InputGroup>
                                        <Input placeholder="0.5" id="StartTime" disabled step="0.5" onChange={(e) => this.handleStartTime(e, this.props.selectedTile)} value={this.props.gridItems[this.props.selectedTile].start_time} type="number"/>
                                        <InputGroupAddon addonType="append">
                                            <InputGroupText className="input-group-button" onClick={(e) => this.handleStartTimePositive(e, this.props.selectedTile)}>+</InputGroupText>
                                            <InputGroupText className="input-group-button" onClick={(e) => this.handleStartTimeNegative(e, this.props.selectedTile)}>-</InputGroupText>
                                        </InputGroupAddon>
                                    </InputGroup>
                                </FormGroup>
                                <FormGroup>
                                    <Label htmlFor="Duration">Duration (Hrs)</Label>
                                    <InputGroup>
                                        <Input placeholder="0.5" id="Duration" disabled step="0.5" onChange={(e) => this.handleDuration(e, this.props.selectedTile)} value={this.props.gridItems[this.props.selectedTile].duration} type="number"/>
                                        <InputGroupAddon addonType="append">
                                            <InputGroupText className="input-group-button" onClick={(e) => this.handleDurationPositive(e, this.props.selectedTile)}>+</InputGroupText>
                                            <InputGroupText className="input-group-button" onClick={(e) => this.handleDurationNegative(e, this.props.selectedTile)}>-</InputGroupText>
                                        </InputGroupAddon>
                                    </InputGroup>
                                </FormGroup>
                                <FormGroup>
                                    <Label htmlFor="Color">Color</Label>
                                    <Input id="Color" onChange={(e) => this.handleColor(e, this.props.selectedTile)} value={this.props.gridItems[this.props.selectedTile].color} type="select">
                                        {colors}
                                    </Input>
                                </FormGroup>
                                <FormGroup className="rightside">
                                    <Button color="info" onClick={(e) => this.handlePlayNow(e, this.props.selectedTile)}>Play Now</Button>&ensp;
                                    <Button color="danger" onClick={(e) => this.handleDelete(e, this.props.selectedTile)}>Delete</Button>
                                </FormGroup>
                            </Form>
                        </TabPane>
                        <TabPane tabId="2">
                            <Form>
                                <FormGroup>
                                    <Label htmlFor="command">Command</Label>
                                    <Input placeholder="command" id="command" maxlength="1000" onChange={(e) => this.handleCommand(e, this.props.selectedTile)} value={this.props.gridItems[this.props.selectedTile].command} type="textarea"/>
                                </FormGroup>
                            </Form>
                        </TabPane>
                    </TabContent>
                </div>
            </div>
        </div>
        ) : (
            <div className="config-panel">
                <div className="config-panel-inner">
                    <div className="form">
                    <h3>No element selected</h3>
                    <h6>Click on a event or time slot to get started</h6>
                    </div>
                </div>
            </div>
        );
    }
}




  export default Editor;