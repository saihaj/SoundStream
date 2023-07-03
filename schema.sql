create table sound_editions
(
    id          text not null PRIMARY KEY,
    "transactionHash"      text not null,
    "deployer"        text not null,
    "blockNumber" bigint not null
);
