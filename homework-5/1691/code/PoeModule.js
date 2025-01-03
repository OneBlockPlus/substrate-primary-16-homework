import React, { useEffect, useState } from 'react'
import {Form, Input, Grid, Card, Statistic, Button, Header, Divider} from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'
import {blake2AsHex} from "@polkadot/util-crypto";

function Main(props) {
    const { api } = useSubstrateState()
    const {accountPair} = props;

  // The transaction submission status
    const [status, setStatus] = useState('')
    const [digest, setDigest] = useState('')
    const [owner, setOwner] = useState('')
    const [newOwner, setNewOwner] = useState('')
    const [blockNumber, setBlockNumber] = useState(0)

    let fileReader ;

    const bufferToDigest = () =>{
        const content = Array.from(new Uint8Array(fileReader.result))
            .map(b => b.toString(16).padStart(2, '0')).join('');

        const hash = blake2AsHex(content, 256);
        setDigest(hash);
    }

    const handleFileChosen = file => {
        fileReader = new FileReader();
        fileReader.onloadend = bufferToDigest;
        fileReader.readAsArrayBuffer(file);
    }

    const handleTxStatusChange = async(status) => {
        setStatus(status);
        console.log(status);
        if(status.includes('Finalized')){
            const result = await api.query.poeModule.proofs(digest);
            console.log(result)
            if(result.isSome){
                const r = result.unwrap();
                console.log(`${r}`);
                setOwner(r[0].toString());
                setBlockNumber(r[1].toNumber());
            }else{
                setOwner('');
                setBlockNumber(undefined);
            }
        }
    }

    useEffect(() => {
        let unsubscribe;

        api.query.poeModule.proofs(digest, (result)=>{
            setOwner(result[0].toString());
            setBlockNumber(result[1].toNumber());
        }).then((unsub)=>{
            unsubscribe = unsub;
        });

    return () => unsubscribe && unsubscribe()
  }, [api.query.poeModule])


    return (
        <Grid stackable columns={2} padded>
            <Grid.Row>
                <Grid.Column width={16}>
                    <Header as="h1" textAlign="center">
                        Proof of Existence
                    </Header>
                    <Card fluid>
                        <Card.Content>
                            <Form>
                                <Form.Field>
                                    <label>Your File</label>
                                    <Input
                                        type="file"
                                        id="file"
                                        onChange={(e) => handleFileChosen(e.target.files[0])}
                                    />
                                </Form.Field>
                                <Form.Field>
                                    <label>New Owner Address</label>
                                    <Input
                                        placeholder="Enter New Owner Address"
                                        value={newOwner}
                                        onChange={(e) => setNewOwner(e.target.value)}
                                    />
                                </Form.Field>
                                <Divider />
                                <Form.Field style={{ textAlign: 'center' }}>
                                    <Button.Group>
                                        <TxButton
                                            accountPair={accountPair}
                                            label="Create Claim"
                                            type="SIGNED-TX"
                                            setStatus={handleTxStatusChange}
                                            attrs={{
                                                palletRpc: 'poeModule',
                                                callable: 'createClaim',
                                                inputParams: [digest],
                                                paramFields: [true],
                                            }}
                                        />
                                        <Button.Or />
                                        <TxButton
                                            accountPair={accountPair}
                                            label="Revoke Claim"
                                            type="SIGNED-TX"
                                            setStatus={handleTxStatusChange}
                                            attrs={{
                                                palletRpc: 'poeModule',
                                                callable: 'revokedClaim',
                                                inputParams: [digest],
                                                paramFields: [true],
                                            }}
                                        />
                                        <Button.Or />
                                        <TxButton
                                            accountPair={accountPair}
                                            label="Transfer Ownership"
                                            type="SIGNED-TX"
                                            setStatus={handleTxStatusChange}
                                            attrs={{
                                                palletRpc: 'poeModule',
                                                callable: 'transferOwnership',
                                                inputParams: [digest, newOwner],
                                                paramFields: [true, true],
                                            }}
                                        />
                                    </Button.Group>
                                </Form.Field>
                            </Form>
                        </Card.Content>
                    </Card>
                </Grid.Column>
                <Grid.Column width={16}>
                    <Grid.Row>
                        <Card fluid>
                            <Card.Content textAlign="center">
                                <Statistic size="mini" label="Digest" value={digest || 'No File Selected'} />
                            </Card.Content>
                        </Card>
                    </Grid.Row>
                    <Grid.Row style={{ marginTop: '1em' }}>
                        <Card fluid>
                            <Card.Content textAlign="center">
                                <Statistic size="mini" label="Owner" value={owner || 'None'} />
                                <Statistic size="mini" label="Block Number" value={blockNumber || 'None'} />
                            </Card.Content>
                        </Card>
                    </Grid.Row>
                    <Grid.Row style={{ marginTop: '1em' }}>
                        <Card fluid>
                            <Card.Content textAlign="center">
                                <Statistic size="mini" label="Status" value={status || 'Idle'} />
                            </Card.Content>
                        </Card>
                    </Grid.Row>
                </Grid.Column>
            </Grid.Row>
        </Grid>
    );
}

export default function PoeModule(props) {
    const { api } = useSubstrateState();
    return api.query.poeModule && api.query.poeModule.proofs ? <Main {...props} /> : null;
}