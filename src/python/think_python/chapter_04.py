from swampy.AmoebaWorld import AmoebaWorld, Amoeba
# create the World
world = AmoebaWorld(interactive=True)
world.set_end_time('2 * math.pi')
world.set_x_t('10 * math.cos(t)')
world.set_y_t('10 * math.sin(t)')
# create the amoeba
amoeba = Amoeba()
# wait for the user
world.mainloop()