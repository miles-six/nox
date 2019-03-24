import * as React from 'react';
import {connect} from 'react-redux';
import {DeployableApp} from "../../../fluence/deployable";
import {defaultContractAddress} from "../../../constants";
import {
    displayLoading,
    hideLoading,
    retrieveApp,
} from '../../actions';
import FluenceCluster from '../fluence-cluster';
import {App, AppId} from "../../../fluence";
import {Action} from "redux";
import {cutId} from "../../../utils";

interface State {
}

interface Props {
    deployState: string | undefined,
    appId: number | undefined,
    app: DeployableApp | undefined,
    apps: {
        [key: string]: App
    },
    trxHash: string,
    retrieveApp: (appId: AppId) => Promise<Action>,
    displayLoading: typeof displayLoading,
    hideLoading: typeof hideLoading,
}

class Snippets extends React.Component<Props, State> {
    state: State = {};

    loadData(): void {
        this.props.displayLoading();
        this.props.retrieveApp(String(this.props.appId)).then(this.props.hideLoading).catch(this.props.hideLoading);
    }

    componentDidUpdate(prevProps: Props): void {
        if (this.props.appId && prevProps.appId != this.props.appId) {
            this.loadData();
        }
    }

    getDeployStateLabel(deployState: string|undefined): string {
        switch (deployState) {
            case 'prepare': {
                return 'preparing transaction...';
            }
            case 'trx': {
                return 'sending transaction...';
            }
            default: {
                return '';
            }
        }
    }

    render(): React.ReactNode {
        if (this.props.app != undefined && this.props.appId != undefined) {
            const appInfo = this.props.apps[this.props.appId];
            return (
                <div className="box box-widget widget-user-2">
                    <div className="widget-user-header bg-fluence-blue-gradient">
                        <div className="widget-user-image">
                            <span className="entity-info-box-icon">
                                <i className="ion ion-ios-checkmark-outline"/>
                            </span>
                        </div>
                        <h3 className="widget-user-username">Connect to {this.props.app.name}</h3>
                        <h3 className="widget-user-desc">appID: <b>{this.props.appId}</b></h3>
                    </div>
                    <div className="box-footer no-padding">
                        <div className="box-body">
                            {this.props.trxHash && <p>Transaction hash: <a href={'https://rinkeby.etherscan.io/tx/' + this.props.trxHash} title={this.props.trxHash} className="etherscan-link" target="_blank">{cutId(this.props.trxHash)}</a></p>}
                            <p>Install dependency:</p>
                            <pre>npm install --save fluence@0.1.18</pre>
                            <p>Connect to {this.props.app.name}, add snippet to a JS file:</p>
                            <pre>{`
import * as fluence from "fluence";

let contract = "${defaultContractAddress}";
fluence.connect(contract, ${this.props.appId}, "http://data.fluence.one:8545").then((s) => {
    console.log("Session created");
    window.session = s;
})
                            `}
                            </pre>
                            <p>Send request:</p>
                            <pre> {`
session.request("CREATE TABLE users(id int, name varchar(128), age int)");
session.request("INSERT INTO users VALUES(1, 'Sara', 23)");
session.request("INSERT INTO users VALUES(2, 'Bob', 19), (3, 'Caroline', 31), (4, 'Max', 25)");
session.request("SELECT AVG(age) FROM users").result().then((r) => {
    console.log("Result: " + r.asString());
});
                            `}
                            </pre>
                            <strong><i className="fa fa-bullseye margin-r-5"></i>Cluster</strong>
                            {appInfo && <FluenceCluster appId={this.props.appId} cluster={appInfo.cluster}/>}
                        </div>
                    </div>
                </div>
            )
        } else if (this.props.deployState) {
            return (
                <div className="box box-widget widget-user-2">
                    <div className="widget-user-header bg-fluence-blue-gradient">
                        <div className="widget-user-image">
                            <span className="entity-info-box-icon"><i className="fa fa-refresh fa-spin"></i></span>
                        </div>
                        <h3 className="widget-user-username">Deploying app</h3>
                        <h3 className="widget-user-desc">...</h3>
                    </div>
                    <div className="box-footer no-padding">
                        <div className="box-body">
                            <p>Status: {this.getDeployStateLabel(this.props.deployState)}</p>
                        </div>
                    </div>
                </div>
            );
        } else {
            return null
        }
    }
}

const mapStateToProps = (state: any) => {
    return {
        deployState: state.deploy.deployState,
        app: state.deploy.app,
        appId: state.deploy.appId,
        trxHash: state.deploy.trxHash,
        apps: state.apps,
    }
};

const mapDispatchToProps = {
    displayLoading,
    hideLoading,
    retrieveApp,
};

export default connect(mapStateToProps, mapDispatchToProps)(Snippets);
