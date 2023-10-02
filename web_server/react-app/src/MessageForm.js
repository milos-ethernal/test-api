// MessageForm.js
import React, { useState, useEffect } from 'react';

function MessageForm() {
    const [messageData, setMessageData] = useState({
        address: '',
        transaction: '',
        policy: '',
        vm_address: '',
    });
    const [receivedMessage, setReceivedMessage] = useState('');
    const [ws, setWs] = useState(null);

    useEffect(() => {
        const newWs = new WebSocket('ws://localhost:19999/ws');
        newWs.onopen = () => {
            console.log('WebSocket connection opened');
        };
        newWs.onmessage = (event) => {
            const received = event.data;
            setReceivedMessage(received);

            let receivedObj = JSON.parse(received)

            // Step 2 WS call || RequestPolicy
            if (receivedObj.type === "requestPolicy") {
                alert("Policy requested " + received)

                // Populate policy id
                let msg = { ...receivedObj.message, policy: "37" }

                // Run server
                let api_msg = { "tx_id": receivedObj.message.transaction, "policy_id": receivedObj.message.policy }
                fetch('http://0.0.0.0:9090/start-server', {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(api_msg),
                })
                    .then((response) => response.json())
                    .then((data) => alert(data.data))
                    .catch((error) => console.error('Error while starting server:', error));

                // Step 3 API call || DeliverPolicy
                fetch('/api/deliverPolicy', {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(msg),
                })
                    .then((response) => response.json())
                    .then((data) => console.log(data))
                    .catch((error) => console.error('Error while fetching:', error));
            }
            // Step 4 WS call || DeliverPolicy
            else if (receivedObj.type === "deliverPolicy") {
                alert("Policy delivered " + received)

                let msg = { "tx_id": receivedObj.message.transaction, "policy_id": receivedObj.message.policy, "to": "0.0.0.0:10501" }
                fetch('http://0.0.0.0:9090/start-client', {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(msg),
                })
                    .then((response) => response.json())
                    .then((data) => alert(data.data))
                    .catch((error) => console.error('Error while starting client:', error));
            }
            else {
                console.log("Unknown type.")
            }
        };
        setWs(newWs);

        return () => {
            newWs.close();
        };
    }, []);

    const handleSubmit = (e) => {
        e.preventDefault();
        // Step 1 WebSocket call || RequestPolicy
        // if (ws) {
        //     ws.send(JSON.stringify(messageData));
        // }

        // Step 1 API call || RequestPolicy
        fetch('/api/requestPolicy', {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(messageData),
        })
            .then((response) => response.json())
            .then((data) => console.log(data))
            .catch((error) => console.error('Error while requesting:', error));

        setMessageData({
            address: '',
            transaction: '',
            policy: '',
            vm_address: '',
        });
    };

    return (
        <div>
            <form onSubmit={handleSubmit}>
                <label>
                    Address (Client IP):
                    <input
                        type="text"
                        value={messageData.address}
                        onChange={(e) =>
                            setMessageData({ ...messageData, address: e.target.value })
                        }
                    />
                </label>
                <label>
                    Transaction:
                    <input
                        type="text"
                        value={messageData.transaction}
                        onChange={(e) =>
                            setMessageData({ ...messageData, transaction: e.target.value })
                        }
                    />
                </label>
                {/* <label>
                Policy:
                <input
                    type="text"
                    value={messageData.policy}
                    onChange={(e) =>
                    setMessageData({ ...messageData, policy: e.target.value })
                    }
                />
                </label> */}
                <label>
                    VM Address:
                    <input
                        type="text"
                        value={messageData.vm_address}
                        onChange={(e) =>
                            setMessageData({ ...messageData, vm_address: e.target.value })
                        }
                    />
                </label>
                <button type="submit">Send</button>
            </form>
            <div>
                <strong>Received Message:</strong> {receivedMessage}
            </div>
        </div>
    );
}

export default MessageForm;
