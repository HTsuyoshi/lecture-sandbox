import numpy as np
# The Basics

# Creating arrays
a = np.array([1,2,3,4,5], dtype='int16')
b = np.array([[1,2,3,4,5,12],[6,7,8,9,10,11]], dtype='int32')

print("Creating arrays")
print(a)
print(b)

# Get dimension
print("Getting dimension")
print(a.ndim)
print(b.ndim)

# Get shape
print("Getting shape")
print(a.shape)
print(b.shape)

# Get Type
print("Getting type")
print(a.dtype)
print(b.dtype)

# Get Size
print("Getting size")
print(a.itemsize)
print(b.itemsize)

# Get Total size
print("Getting total size")
print(a.nbytes)
print(b.nbytes)

# Acess elements
print("Elements")
print(a[0])
print(b[0][-1])

print("Columns")
print(b[0,:])
print(b[:,0])
print(b[0,1:6:2])

# Set a column to 5
b[0,:] = 5
print(b)

# Set a custom column to 5
b[0,1:4] = [1,2,3]
print(b)

# Zero matrix
np.zeros((2,3))

# One matrix
np.ones((2,3), dtype='int32')

# Any other number
np.full((2,2), 12)

# Any other number size of matrix a
np.full_like(a, 12)

# Random decimal numbers
np.random.rand(4,2)

# Random integer values
np.random.randint(7, size=(3,3))

# Indentity matrix
np.identity(5)

# Repeat line
arr = np.array([1,2,3])
r1 = np.repeat(arr, 3, axis=0)
r1 = np.repeat(arr, 3, axis=1)

# Copying arrays
a = np.zeros(3)
b = a.copy()

# Mathemathics
# Sum every element + 2
a + 2

# Sum two arrays
a + b

# Take sin of every element
np.cos(a)

# Linear Algegra
# Multiply matrix
np.matmul(a,b)

np.linalg.det(a)

# Statistics
np.min(a, axis=1)
np.max(a)

np.sum(a, axis=1)

# Reorganizing Arrays
a = np.array([1,3,2,4])
a.reshape(2,2)

a = np.array([1,2,3,4,5])
b = np.array([6,7,8,9,10])

np.vstack([a,b])
np.hstack((a,b))

# From file
filedata = np.genfromtxt('data.txt', delimiter=',')
filedata.astype('int32')

# Boolean Masking and Advancing Indexing
# For every element it verify if its bigger than 50
filedata > 50

# Get only elements that is bigger than 50
filedata[filedata > 50]
filedata[filedata > 50]
(filedata > 50) & (filedata < 100)

# Index a list in Numpy
a = np.array([1,2,3,4,5,6,7,8,9])
a[[5,3,7,8]]

