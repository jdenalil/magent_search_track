from pettingzoo import mpe
from magent_autonomy import return_one

def setup_env():
    return mpe.simple_spread_v3.parallel_env(N=3, local_ratio=0.5, max_cycles=50)

if __name__ == "__main__":
    env = setup_env()
    observations, infos = env.reset()

    while env.agents:

        actions = {agent: return_one() for agent in env.agents}
        #print(actions)

        observations, rewards, terminations, truncations, infos = env.step(actions)

        #print(observations)

    env.close()
