from pettingzoo import mpe
from magent_autonomy import towards_landmark
import time

def setup_env():
    return mpe.simple_spread_v3.parallel_env(N=3, local_ratio=0.0, max_cycles=50, render_mode="human")

if __name__ == "__main__":
    env = setup_env()
    observations, infos = env.reset()

    while env.agents:
        # assemble actions for each agent
        actions = {}
        for agent in env.agents:
            # TODO: add target assignment based on distance to targets
            actions[agent] = towards_landmark(
                observations[agent].tolist(), int(agent[-1])
            )
        observations, rewards, terminations, truncations, infos = env.step(actions)
        # print(rewards)
        # time.sleep(1)

    env.close()
