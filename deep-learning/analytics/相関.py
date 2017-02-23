# qiita.com/ynakayama/items/1a55ddbb85ae08970ad8
import pandas as pd
from pandas.tools.plotting import scatter_matrix
import matplotlib.pyplot as plt
import scipy.stats as sp

data = pd.read_csv("data.csv", names=['X', 'Y', 'Z'])
print data.describe()


plt.figure()
scatter_matrix(data)
plt.savefig("image.png")

print data.corr()

x = data.ix[:,0].values
y = data.ix[:,1].values
z = data.ix[:,2].values

print x
print y
print z

slope, intercept, r_value, p_value, std_err = sp.stats.linregress(x, z)
print(slope, intercept, r_value)

slope, intercept, r_value, p_value, std_err = sp.stats.linregress(y, z)
print(slope, intercept, r_value)