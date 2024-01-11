import { newBet as newBetEvent } from "../generated/CoinFlip/CoinFlip"
import { newBet } from "../generated/schema"

export function handlenewBet(event: newBetEvent): void {
  let entity = new newBet(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity._str = event.params._str

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}
