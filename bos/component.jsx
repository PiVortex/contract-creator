const Input = styled.input`
  margin: 5px;
  padding: 8px;
  height: 40px;
`;

const Button = styled.button`
  margin: 5px;
  padding: 8px;
  width: 150px;
  background-color: #d3d3d3;
  color: #151515;
  cursor: pointer;
  border: none;
`;

const Container = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
`;

const contract = "contractcreator.testnet";
const method = "create_factory_subaccount_and_deploy";
const gas = 300 * Math.pow(10, 12);
const deposit = 5 * Math.pow(10, 24);

const isSignedIn = context.accountId;


State.init({
    contract_name,
    contract_type
  });

const handleClickGreeting = () => {
    let args = {
        name: state.contract_name,
        contract_type: "Greeting"
    }

    Near.call(contract, method, args, gas, deposit);
};

const handleClickDonation = () => {
    let args = {
        name: state.contract_name,
        contract_type: "Donation"
    }

    Near.call(contract, method, args, gas, deposit);
}


return (
    <div>
        <h1> Example Contract deployer </h1>
        <h3> Enter contract name </h3>

    <Container>
        <Input
            type="text"
            id="contract_name"
            value={state.contract_name}
            onChange={(e) => {
                State.update({ [e.target.id]: e.target.value });
            }}
        />
        <Button type="submit" onClick={(handleClickGreeting)} disabled={!isSignedIn}>
            Greeting Contract
        </Button>
        <Button
        className="btn-no"
        onClick={handleClickDonation}
        disabled={!isSignedIn}
        >
            Donation Contract
        </Button>
  </Container>
    <p>Deploying to {state.contract_name}.contractcreator.testnet</p>

    </div>
  );