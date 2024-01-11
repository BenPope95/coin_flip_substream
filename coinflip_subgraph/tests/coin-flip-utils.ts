import { newMockEvent } from "matchstick-as"
import { ethereum } from "@graphprotocol/graph-ts"
import { newBet } from "../generated/CoinFlip/CoinFlip"

export function createnewBetEvent(_str: string): newBet {
  let newBetEvent = changetype<newBet>(newMockEvent())

  newBetEvent.parameters = new Array()

  newBetEvent.parameters.push(
    new ethereum.EventParam("_str", ethereum.Value.fromString(_str))
  )

  return newBetEvent
}
